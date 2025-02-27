use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use anyhow::{Context, anyhow};
use config::Config;
use log::{LevelFilter, error, info};

// Type alias for tokio return types
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use lc_signage::{ConnectionData, LcSignage};

/// fn main
///
/// Sets up journaling
/// Reads configuration file
/// Initializes core program logic
/// Loops every 15 minutes

#[tokio::main]
async fn main() -> Result<()> {
    // setup systemd journal logging
    install_logger();

    log::set_max_level(LevelFilter::Info);

    let config_location = home::home_dir().unwrap();

    let cfg = Config::builder().add_source(config::File::with_name(
        config_location
            .join(".config/lc-signage/config.toml")
            .to_str()
            .ok_or(anyhow!("config file not located"))?,
    ));

    match cfg.build() {
        Ok(cfg) => {
            let oauth_url = cfg.get_string("auth_url").context("expected oauth url")?;
            let token_url = cfg.get_string("token_url").context("expected token url")?;
            let feed_url = cfg.get_string("feed_url").context("expected feed url")?;
            let client_id = cfg.get_string("client_id").context("expected client id")?;
            let client_secret = cfg
                .get_string("client_secret")
                .context("expected client secret")?;
            let room_keys = cfg
                .get_array("room_keys")
                .context("expected room keys")?
                .into_iter()
                .map(|r| r.to_string())
                .collect();
            let save_path = match cfg.get_string("save_path") {
                Ok(path) => Some(path),
                _ => None,
            };
            let query_start_time = cfg.get_string("start_time").unwrap_or("now".into());
            let query_end_time = cfg.get_string("end_time").unwrap_or("tomorrow".into());

            let query_url = format!("{feed_url}?start={query_start_time}&end={query_end_time}");

            // setup service structs
            let connection = ConnectionData::new(
                oauth_url,
                token_url,
                query_url,
                client_id,
                client_secret,
                save_path,
            );
            let mut lc_signage = LcSignage::new(room_keys, connection);

            // sleep process in 15 min intervals
            let sleep_interval = 900;
            let interval = Duration::from_secs(sleep_interval);
            let mut next_time = Instant::now() + interval;

            loop {
                let start_time = Instant::now();

                match lc_signage.process_events().await {
                    Ok(r) => {
                        let total_time = start_time.elapsed().as_secs_f32();
                        info!(
                            "success: total {:.4}s | json {:.4}s | processing {:.4}s",
                            total_time,
                            r,
                            total_time - r
                        );
                    }
                    Err(e) => error!("error encountered: {:?}", e),
                }

                sleep(next_time - Instant::now());
                next_time += interval;
            }
        }
        Err(e) => {
            error!("Fatal error - config file not found. {:?}", e);
        }
    }

    Ok(())
}

#[cfg(feature = "cross_platform")]
fn install_logger() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
}

#[cfg(feature = "systemd")]
fn install_logger() {
    systemd_journal_logger::JournalLog::new()
        .unwrap()
        .install()
        .unwrap();
}
