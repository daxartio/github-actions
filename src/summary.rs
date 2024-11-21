use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug)]
pub enum SummaryError {
    VarError(env::VarError),
    FileError(io::Error),
}

fn clean_markdown_string(markdown_text: &str) -> String {
    markdown_text
        .replace("%25", "%")
        .replace("%0D", "\r")
        .replace("%0A", "\n")
        .to_string()
}

pub fn append_job_summary(markdown_text: &str) -> Result<(), SummaryError> {
    let summary_path =
        env::var(crate::GITHUB_STEP_SUMMARY).map_err(|err| SummaryError::VarError(err))?;
    let mut file = OpenOptions::new()
        .append(true)
        .open(summary_path)
        .map_err(|err| SummaryError::FileError(err))?;
    writeln!(file, "{}", clean_markdown_string(markdown_text))
        .map_err(|err| SummaryError::FileError(err))?;
    Ok(())
}

pub fn overwrite_job_summary(markdown_text: &str) -> Result<(), SummaryError> {
    let summary_path =
        env::var(crate::GITHUB_STEP_SUMMARY).map_err(|err| SummaryError::VarError(err))?;
    let mut file = File::create(summary_path).map_err(|err| SummaryError::FileError(err))?;
    writeln!(file, "{}", clean_markdown_string(markdown_text))
        .map_err(|err| SummaryError::FileError(err))?;
    Ok(())
}

pub fn remove_job_summary() -> io::Result<()> {
    if let Ok(summary_path) = env::var(crate::GITHUB_STEP_SUMMARY) {
        let path = Path::new(&summary_path);
        if path.exists() {
            std::fs::remove_file(path)?;
        }
    }
    Ok(())
}
