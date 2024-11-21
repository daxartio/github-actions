# Github Actions

A Rust crate for building GitHub Actions with ease, offering utilities to handle inputs, outputs, logging, and more.

## Features

- Parse inputs (`get_input`, `get_bool_input`, `get_multiline_input`).
- Manage GitHub Actions state (`get_state`, `save_state`).
- Log messages with various levels (`info!`, `debug!`, `warn!`, `error!`, `notice!`).
- Handle sensitive data with `set_secret`.
- Manage environment variables with `export_variable` and `add_path`.
- Control GitHub Actions commands (`stop_commands!`).
- Group logs for better readability (`group!`).
- Adding a job summary (`append_job_summary`, `overwrite_job_summary`, `remove_job_summary`)

## Installation

```
cargo add github-actions
```

## Documentation

For more detailed information, visit the [API Documentation](https://docs.github.com/en/actions/writing-workflows).

---

This crate simplifies interactions with GitHub Actions, enabling quick and clean integration into your Rust-based workflows.
