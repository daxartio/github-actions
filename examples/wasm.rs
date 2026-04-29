use github_actions::*;

fn main() -> Result<(), FileCommandError> {
    let name = get_input("name").unwrap_or_else(|_| "wasm".to_string());
    let dry_run = get_bool_input("dry_run").unwrap_or_default();

    group!("wasm github-actions example");
    info!("hello {}", name);
    info!("debug={}", is_debug());
    info!("dry_run={}", dry_run);
    notice!("running through wasmer");

    set_secret("wasm-secret");
    export_variable("WASM_EXAMPLE", "ok")?;
    set_output("wasm-example", "ok")?;

    Ok(())
}
