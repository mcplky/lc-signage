# lc-signage

Display signage backend service for Library Calendar-driven websites. Takes event information from the calendar and processes it into smaller individual `.json` files for a frontend webpage or other service.

`lc-signage` is developed and maintained by the Madion County Public Library Technology Services department.

## Package Information

- lc-signage: Rust client service to poll website for event information.
- lc-signage/frontend: Example HTML, CSS, and JavaScript for a server deployment.
- lc-signage/hardware: Information about the equipment and hardware used to create a working display sign.

See the subdirectories for additional information on frontend and hardware. All three components are able to be used without the other if desired; the exported JSON from the service is a stripped-down and minimalized version of the JSON returned by Library Calendar.

## Installation

`lc-signage` runs on any Linux or Windows-based platform with the default configuration of `cross_platform`. If you are running on a Linux distribution with `systemd`, you may wish to enable the `systemd` feature to enable native logging to `journald`. The `env_logger` backend for `cross_platform` will forward its output to `journalctl` if the service is installed as a `systemd` service unit.

Download the current release from the [releases](https://github.com/mcplky/lc-signage/releases/latest) and extract the binary to your desired path. The binary can be run as-is as long as the configuration file is present.

### Building from source

`cargo run --release` will compile and run the program immediately for testing purposes, as long as `config.toml` exists and is correctly configured.

Install `lc-signage` to your default `cargo` binary directory by invoking `cargo install --path=.`. You may also install to a different path if desired.

### Systemd Unit Configuration

Running `lc-signage` as a system service enables auto-restart of service in case of failure and lets the service start on login. The `ExecStart` option points to the install path, so make sure it points to your binary. This step is not necessary but it simplifies deployment.

For an example `lc-signage.service` unit file (located in `~/.config/systemd/user/`):

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

Check the log status by running:

```bash
journalctl --user -u lc-signage
```

You will need to ensure that you maintain a login on the server you install this on to run the application, so consider an autologin configuration.

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
# Optional parameter for save location
# Default is given below
save_path = "~/.local/share/web/events"

# Optional parameters for date range to pull
# Default configuration only pulls programs that have an end time between the current time and the end of the current day
start_time = "now"
end_time = "tomorrow"
```

`save_path` is an optional field; it will overwrite the default save location. Your user needs write permissions on the target output directory. The program will create the directory if it does not exist already.

## Limitations

Currently `lc-signage` is limited in scope and is designed for our use case. It will only parse out the following information from a calendar event:

* Title
* Public/Private
* Start date
* End date
* Branch
* Room
* Event ID
* Moderation State

The output JSON contains:

* Title
* Public/Private
* Date (in `%Y-%m-%d` format)
* Start time (in `%h:%m %p` format)
* End time (in `%h:%m %p`)
* Event ID
* Room
* Moderation State

The output JSON is stored in the directory specified by `save_path` in the config file.

The service should be compatible as-is with any Library Calendar-based event calendar system. If you require additional functionality please feel free to open an issue on the tracker or make the modification on your local copy.
