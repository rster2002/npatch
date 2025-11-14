use std::path::{Path, PathBuf};
use std::process::Command;
use regex::Regex;
use crate::modules::lib::error::LibError;

pub fn dependencies_of<P: AsRef<Path>>(binary_path: P) -> Result<Vec<PathBuf>, LibError> {
    if cfg!(target_os = "macos") {
        otool_dependencies_of(binary_path.as_ref())
    } else {
        unimplemented!()
    }
}

pub fn otool_dependencies_of(binary_path: &Path) -> Result<Vec<PathBuf>, LibError> {
    let command_output = Command::new("otool")
        .arg("-L")
        .arg(binary_path)
        .output()
        .map_err(|e| LibError::FailedToRunOTool(e))?;

    let regex = Regex::new(r#"^\s+(\S+)"#)
        .unwrap();

    let output_string = String::from_utf8(command_output.stdout)?;

    let mut output = Vec::new();

    let mut lines = output_string.lines();
    lines.next();

    for line in lines {
        let captures = regex.captures(line)
            .ok_or(LibError::FailedToParseOToolOutput)?;

        let name = captures.get(1)
            .ok_or(LibError::FailedToParseOToolOutput)?
            .as_str();

        output.push(PathBuf::from(name));
    }

    Ok(output)
}