# lc-signage

Display signage backend service for Library Calendar-driven websites. Takes event information from the calendar and processes it into smaller individual `.json` files for a frontend webpage or other service.

## Installation

`lc-signage` runs on any Linux distribution with `systemd`. Support for non-`systemd` distributions is planned.

### Building from source

Install `lc-signage` to your default `cargo` binary directory by invoking `cargo install --path=.`. Afterwards, create a `systemd` user unit in `~/.config/systemd/user/lc-signage.service` with the following format:

```
[Unit]
Description=Library Calendar Room Display Service

[Service]
Restart=on-failure
RestartSec=5s
ExecStart=/home/<user>/.cargo/bin/lc-signage

[Install]
WantedBy=default.target
```

Afterwards, you can refresh the systemd service cache and enable the service and start it:

```bash
systemctl --user daemon-reload
systemctl --user enable --now lc-signage.service
```

You will need to ensure that you maintain a login on the server you install this on to run the application.

## Configuration

`lc-signage` is configured through `config.toml` located by default in your user directory at `~/.config/lc-signage/config.toml`. Currently this is the only supported location to configure this service.

```toml
auth_url = "https://<your library url>/oauth/authorize"
token_url = "https://<your library url>/oauth/token"
feed_url = "https://<your library url>/events/feed/json"
client_id = "<id string>"
client_secret = "<secret string>"
username = "<username>"
password = "<password>"
# Internal room IDs.
# Use '+' to separate rooms in the same string if you want their output to be merged in the same JSON result file
room_keys = ["123", "456", "789+102"] 
```

## Limitations

Currently `lc-signage` is very limited in scope and is designed for our use case. It will only parse out the following information from a calendar event:

* Title
* Public/Private
* Start date
* End date
* Branch
* Room
* Event ID

The output JSON contains an even more limited subset:

* Title
* Start time
* End time
* Event ID
* Room

The service should be compatible as-is with any Library Calendar-based event calendar system. If you require additional functionality please feel free to open an issue on the tracker or make the modification on your local copy.
