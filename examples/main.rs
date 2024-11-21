use github_actions::*;

fn main() {
    info!("debug={}", is_debug());
    info!(get_input("name").unwrap_or("default".to_string()));
    info!(get_bool_input("bool").unwrap_or_default());
    info!(get_multiline_input("multiline")
        .unwrap_or_default()
        .join("\\n"));
    info!("state={}", get_state("name").unwrap_or_default());

    {
        stop_commands!("token");
        info!("Message")
    }

    {
        stop_commands!();
        info!("Message")
    }

    group!("Main logs");

    info!("Info");
    debug!("Debug");
    error!(format!("Error {}", 1), title: "Title");
    error!("message", title: "title", file: "file", col: 1, end_column: 1, line: 1, end_line: 1);
    warn!("Warning");
    notice!("Notice");

    set_secret("secret");

    export_variable("key", format!("value {}", "env")).unwrap();
    set_output("key", "value").unwrap();
    save_state("key", "value").unwrap();

    add_path("path").unwrap();
}
