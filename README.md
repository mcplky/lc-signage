# lc-signage

Display signage backend service for Library Calendar-driven websites. Takes event information from the calendar and processes it into smaller individual `.json` files for a frontend webpage or other service.

## Installation

`lc-signage` runs on any Linux distribution with `systemd`. `systemd` support is only needed for logging to `journald`. We plan to enable a config option for logging for alternative init systems like `runit`, `openrc` and `sysvinit`.

Download the current release from the [/releases/latest](Releases) and extract the binary to your desired path. The binary can be run as-is as long as the configuration file is present.

### Building from source

Install `lc-signage` to your default `cargo` binary directory by invoking `cargo install --path=.`. You may also install to a different path if desired.

### Systemd Unit Configuration

Running `lc-signage` as a system service enables auto-restart of service in case of failure and lets the service start on login. The `ExecStart` option points to the install path, so make sure it points to your binary.

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

`lc-signage` is configured through a `config.toml` located by default in your user directory at `~/.config/lc-signage/config.toml`. Currently this is the only supported location to configure this service.

An example configuration file is noted below.

```toml
auth_url = "https://<your library url>/oauth/authorize"
token_url = "https://<your library url>/oauth/token"
feed_url = "https://<your library url>/events/feed/json"
client_id = "<id string>"
client_secret = "<secret string>"
# Internal room IDs.
# Use '+' to separate rooms in the same string if you want their output to be merged in the same JSON result file
room_keys = ["123", "456", "789+102"] 
save_path = "<path to output directory>"
```

`save_path` is an optional field; it will overwrite the default save location. Your user needs write permissions on the target output directory. The program will create the directory if it does not exist already.

## Limitations

Currently `lc-signage` is very limited in scope and is designed for our use case. It will only parse out the following information from a calendar event:

* Title
* Public/Private
* Start date
* End date
* Branch
* Room
* Event ID
* Moderation State

The output JSON contains an even more limited subset:

* Title
* Public/Private
* Start time
* End time
* Event ID
* Room
* Moderation State

The output JSON is stored by default in `~/.local/share/web/events`. If the optional `save_path` value is set in the configuration, that path is used instead.

The service should be compatible as-is with any Library Calendar-based event calendar system. If you require additional functionality please feel free to open an issue on the tracker or make the modification on your local copy.
