# smplFetch 🍎

> A simple system information CLI tool for Linux written in Rust.

smplFetch displays your current time, user, distro, kernel architecture, memory usage, and battery percentage, with optional color output and flexible display modes.

---

## Installation

Install via Cargo:

```bash
cargo install smplfetch
```

Or clone and build:

```bash
git clone https://github.com/DriftingOtter/smplfetch_rs.git
cd smplfetch_rs
cargo build --release
cp target/release/smplfetch /usr/local/bin/
```

---

## Usage

```bash
smplfetch [OPTIONS]
```

### Options

| Short | Long            | Description                                          |
| ----- | --------------- | ---------------------------------------------------- |
| `-c`  | `--color-strip` | Display an 8-color strip (blocks of two characters). |
| `-b`  | `--no-battery`  | Omit the battery information line.                   |
| `-j`  | `--json`        | Output all information in JSON format.               |
| `-m`  | `--minimal`     | Show only Time, User, and Memory usage.              |
| `-h`  | `--help`        | Print help information.                              |
| `-V`  | `--version`     | Print version information.                           |

---

### Examples

* Default human-readable output:

  ```bash
  smplfetch
  ```

* Show color strip with default info:

  ```bash
  smplfetch -c
  ```

* Minimal view (time, user, memory):

  ```bash
  smplfetch -m
  ```

* JSON output, omitting battery:

  ```bash
  smplfetch -j -b
  ```

---

## Authors

* Daksh Kaul // DriftingOtter 🦦 
---

## License

This project is licensed under the [GPL-2.0-or-later](LICENSE) license.

