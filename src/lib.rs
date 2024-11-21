mod command;
mod file_command;
mod group;
mod input;
mod state;
mod stop_commands;
mod summary;
mod vars;

mod macros;

pub use command::*;
pub use file_command::*;
pub use group::*;
pub use input::*;
pub use state::*;
pub use stop_commands::*;
pub use summary::*;
pub use vars::*;

#[cfg(windows)]
const EOL: &'static str = "\r\n";
#[cfg(not(windows))]
const EOL: &'static str = "\n";

fn delimiter() -> String {
    format!("ghadelimiter_{}", uuid::Uuid::new_v4().to_string())
}

fn does_env_exist(envname: &str) -> bool {
    std::env::var(envname).is_ok()
}

pub fn is_debug() -> bool {
    std::env::var(RUNNER_DEBUG).unwrap_or_default() == "1"
}

pub fn export_variable<K, V>(key: K, value: V) -> Result<(), FileCommandError>
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    if does_env_exist(crate::vars::GITHUB_ENV) {
        issue_file_command(
            crate::vars::GITHUB_ENV,
            &prepare_key_value_message(key, value.as_ref(), delimiter().as_str()).unwrap(),
        )
    } else {
        println!(
            "{}",
            Command {
                command: "set-env",
                value: value.as_ref(),
                properties: Some([("name", key.as_ref())].into())
            }
        );
        Ok(())
    }
}

pub fn set_output<K, V>(key: K, value: V) -> Result<(), FileCommandError>
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    if does_env_exist(crate::vars::GITHUB_OUTPUT) {
        issue_file_command(
            crate::vars::GITHUB_OUTPUT,
            &prepare_key_value_message(key, value.as_ref(), delimiter().as_str()).unwrap(),
        )
    } else {
        println!(
            "{}",
            Command {
                command: "set-output",
                value: value.as_ref(),
                properties: Some([("name", key.as_ref())].into())
            }
        );
        Ok(())
    }
}

pub fn save_state<K, V>(key: K, value: V) -> Result<(), FileCommandError>
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    if does_env_exist(crate::vars::GITHUB_STATE) {
        issue_file_command(
            crate::vars::GITHUB_STATE,
            &prepare_key_value_message(key, value.as_ref(), delimiter().as_str()).unwrap(),
        )
    } else {
        println!(
            "{}",
            Command {
                command: "save-state",
                value: value.as_ref(),
                properties: Some([("name", key.as_ref())].into())
            }
        );
        Ok(())
    }
}

pub fn add_path<P>(path: P) -> Result<(), FileCommandError>
where
    P: AsRef<str>,
{
    if does_env_exist(crate::vars::GITHUB_PATH) {
        issue_file_command(crate::vars::GITHUB_PATH, path.as_ref())
    } else {
        println!(
            "{}",
            Command {
                command: "add-path",
                value: path.as_ref(),
                properties: None,
            }
        );
        Ok(())
    }
}

pub fn set_secret<S>(secret: S)
where
    S: AsRef<str>,
{
    println!(
        "{}",
        Command {
            command: "add-mask",
            value: secret.as_ref(),
            properties: None,
        }
    )
}
