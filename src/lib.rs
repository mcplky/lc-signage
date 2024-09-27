use std::{
    collections::HashMap,
    fs::{self, File},
    io::prelude::*,
    path::PathBuf,
    time::Instant,
};

use anyhow::Context;
use bytes::Bytes;
use chrono::{NaiveDate, NaiveTime};
use home::home_dir;
use http_body_util::{BodyExt, Full};
use hyper::{body::Buf, Method, Request};
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use log::error;
use oauth2::{
    basic::{BasicClient, BasicTokenType},
    curl::http_client,
    AuthUrl, ClientId, ClientSecret, EmptyExtraTokenFields, ResourceOwnerPassword,
    ResourceOwnerUsername, StandardTokenResponse, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Type alias for tokio return types to handle asynchronous code correctly
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// `Token`
///
/// Struct for the API token
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct Token {
    t_type: Value,
    expires_in: Value,
    access: Value,
}

/// `LcEvent`
///
/// Struct for all the fields of the JSON we need for our purposes.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct LcEvent {
    title: String,
    public: bool,
    start_date: String,
    end_date: String,
    branch: Value,
    room: Value,
    id: String,
    moderation_state: String,
}

/// `OutputEvent`
///
/// Struct for the essential fields for the display units outside rooms.
#[derive(Debug, Serialize, Deserialize)]
struct OutputEvent {
    title: String,
    public: bool,
    start_time: String,
    end_time: String,
    id: String,
    room: String,
    moderation_state: String,
}

/// `ConnectionData`
///
/// Struct for configuration information that is used to build requests.
pub struct ConnectionData {
    oauth_url: String,
    token_url: String,
    feed_url: String,
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
    access_token: String,
}

impl ConnectionData {
    #[must_use]
    pub fn new(
        oauth_url: String,
        token_url: String,
        feed_url: String,
        client_id: String,
        client_secret: String,
        username: String,
        password: String,
    ) -> Self {
        Self {
            oauth_url,
            token_url,
            feed_url,
            client_id,
            client_secret,
            username,
            password,
            access_token: String::new(),
        }
    }

    /// fn `fetch_json`
    ///
    /// Contact the provided URL to acquire a JSON object, and then return that JSON as a parsed
    /// Rust object as a Vec<LcEvent>.
    pub(crate) async fn fetch_json(&mut self, room: &str) -> Result<Vec<LcEvent>> {
        let https = HttpsConnector::new();
        let client = Client::builder(TokioExecutor::new()).build(https);

        let request = self.make_request(room)?;

        // Fetch the url...
        let response = client.request(request).await?;
        let body = response.collect().await?.aggregate();

        Ok(serde_json::from_reader(body.reader())?)
    }

    /// fn `fetch_api_key()`
    ///
    /// Blocking function to retrieve the API key from the OAuth granter
    /// Uses auth keys and requirements given by Library Market
    pub(crate) fn fetch_api_key(
        &self,
    ) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> {
        let client = BasicClient::new(
            ClientId::new(self.client_id.clone()),
            Some(ClientSecret::new(self.client_secret.clone())),
            AuthUrl::new(self.oauth_url.clone())?,
            Some(TokenUrl::new(self.token_url.clone())?),
        );

        let token_result = client
            .exchange_password(
                &ResourceOwnerUsername::new(self.username.clone()),
                &ResourceOwnerPassword::new(self.password.clone()),
            )
            .request(http_client)?;

        Ok(token_result)
    }

    /// fn `make_request`
    ///
    /// Produces the request for the feed JSON with the appropriate authorization token.
    /// Encodes the secret auth token information until it is consumed by `fetch_json`
    fn make_request(&mut self, room: &str) -> Result<Request<Full<Bytes>>> {
        let url = if room.contains('+') {
            let mut room_split = room.split('+');
            let first_room = room_split
                .next()
                .context("expected a room number in substring")?;
            let mut room_str = format!("?rooms[{first_room}]={first_room}");
            for rm in room_split {
                room_str = format!("{room_str}&rooms[{rm}]={rm}");
            }
            format!("{}{}", self.feed_url, room_str)
        } else {
            format!("{}?rooms[{}]={}", self.feed_url, room, room)
        };

        let result = Request::builder()
            .method(Method::GET)
            .uri(url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .body(Full::new(Bytes::from("hello")))?;

        Ok(result)
    }
}

/// `LcSignage`
///
/// Core part of the process. `LcSignage` maintains information about
pub struct LcSignage {
    room_keys: Vec<String>,
    processed_events: HashMap<String, Vec<OutputEvent>>,
    connection: ConnectionData,
}

impl LcSignage {
    #[must_use]
    pub fn new(room_keys: Vec<String>, connection: ConnectionData) -> Self {
        Self {
            room_keys,
            processed_events: HashMap::new(),
            connection,
        }
    }
    /// fn `process_events()`
    ///
    /// core program loop, abstracted from main to enable recoverable error handling
    /// `room_keys` will need to be modified if a room is to be given a display
    ///
    /// # Errors
    ///
    /// Will return an error in circumstances such as:
    /// server unreachable
    /// JSON incorrect or malformed
    /// unable to parse the JSON to `LcEvent`
    pub async fn process_events(&mut self) -> Result<f32> {
        let mut response_time = 0.;

        // retrieve access key before update loop begins
        self.connection
            .fetch_api_key()?
            .access_token()
            .secret()
            .clone_into(&mut self.connection.access_token);

        for room in &self.room_keys {
            let request_start = Instant::now();
            // in-process error handling to prevent json access issues from breaking a full refresh cycle
            let received_events = match self.connection.fetch_json(room).await {
                Ok(ev) => ev,
                Err(e) => {
                    error!("error encountered in room {}: {}", room, e);
                    continue;
                }
            };
            response_time += request_start.elapsed().as_secs_f32();
            self.processed_events.insert(
                room.into(),
                LcSignage::generate_room_events(received_events)?,
            );
        }

        self.write_output_json()?;

        // clear data structures for next cycle
        // retains allocated memory
        self.processed_events.clear();

        Ok(response_time)
    }

    /// fn `generate_room_events`
    ///
    /// Processes and consumes a `Vec` of `LcEvents`. This should deallocate the memory used for the received
    /// JSON and processes it into a keyed `HashMap` for further processing.
    ///
    /// The `HashMap` is keyed by the room ID number and is dynamically generated.
    fn generate_room_events(events: Vec<LcEvent>) -> Result<Vec<OutputEvent>> {
        let mut publish_events = vec![];
        let today = chrono::Local::now().date_naive();

        for event in events {
            if event.moderation_state == "cancelled".to_string() {
                continue;
            }

            let mut time_str = event.start_date.split_whitespace();
            let scheduled_date = NaiveDate::parse_from_str(
                time_str.next().ok_or("could not read JSON time/date")?,
                "%Y-%m-%d",
            )?;

            if scheduled_date == today {
                let start_time = NaiveTime::parse_from_str(
                    time_str.next().ok_or("could not read JSON time/date")?,
                    "%H:%M:%S",
                )?;

                let end_time = NaiveTime::parse_from_str(
                    event
                        .end_date
                        .split_whitespace()
                        .nth(1)
                        .ok_or("could not split end date string")?,
                    "%H:%M:%S",
                )?;

                publish_events.push(OutputEvent {
                    title: event.title,
                    public: event.public,
                    start_time: start_time.format("%l:%M %p").to_string(),
                    end_time: end_time.format("%l:%M %p").to_string(),
                    room: event
                        .room
                        .as_object()
                        .unwrap()
                        .keys()
                        .next()
                        .unwrap()
                        .to_owned(),
                    id: event.id,
                    moderation_state: event.moderation_state,
                });
            } else {
                break;
            }
        }

        Ok(publish_events)
    }

    /// fn `write_output_json`
    ///
    /// Takes the `HashMap` of `OutputEvents` and consumes it to write the JSON files to disk.
    /// JSON files are written using the internal room ID code as filenames
    /// eg 123.json
    ///
    /// The `HashMap` is converted to an iterator; we currently are not using key-based lookups
    fn write_output_json(&self) -> Result<()> {
        let folder_path: PathBuf = [
            home_dir().ok_or("could not find home directory")?,
            ".local".into(),
            "share".into(),
            "web".into(),
            "events".into(),
        ]
        .iter()
        .collect();

        if !folder_path.exists() {
            fs::create_dir_all(folder_path)?;
        }

        for room in &self.room_keys {
            let save_path: PathBuf = [
                home_dir().ok_or("could not find home directory")?,
                ".local".into(),
                "share".into(),
                "web".into(),
                "events".into(),
                format!("{room}.json").into(),
            ]
            .iter()
            .collect();

            let mut save = File::options()
                .read(false)
                .write(true)
                .create(true)
                .truncate(true)
                .open(save_path)?;

            let json = if self.processed_events.contains_key(&room.to_string()) {
                serde_json::to_string(self.processed_events.get(&room.to_string()).unwrap())?
                    .to_string()
            } else {
                String::new()
            };

            write!(save, "{json}")?;
        }

        Ok(())
    }
}
