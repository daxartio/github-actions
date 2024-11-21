use std::env;
use std::fmt;
use std::fs::OpenOptions;
use std::io::{self, Write};

#[derive(Debug)]
pub enum FileCommandError {
    VarError(env::VarError),
    FileError(io::Error),
}

pub fn issue_file_command(env: &str, message: &str) -> Result<(), FileCommandError> {
    let file = env::var(env).map_err(|err| FileCommandError::VarError(err))?;

    let mut file = OpenOptions::new()
        .append(true)
        .open(file)
        .map_err(|err| FileCommandError::FileError(err))?;

    writeln!(file, "{}", message).map_err(|err| FileCommandError::FileError(err))?;
    Ok(())
}

pub fn prepare_key_value_message<K, V>(key: K, value: V, delimiter: &str) -> Result<String, String>
where
    K: AsRef<str>,
    V: fmt::Display,
{
    let value = value.to_string();

    if key.as_ref().contains(&delimiter) {
        return Err(format!(
            "Unexpected input: key should not contain the delimiter \"{}\"",
            delimiter
        ));
    }

    if value.contains(&delimiter) {
        return Err(format!(
            "Unexpected input: value should not contain the delimiter \"{}\"",
            delimiter
        ));
    }

    Ok(format!(
        "{}<<{}{}{}{}{}",
        key.as_ref(),
        delimiter,
        crate::EOL,
        value,
        crate::EOL,
        delimiter
    ))
}
