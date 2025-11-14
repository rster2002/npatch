use std::fs::read_dir;
use std::path::{Path, PathBuf};
use crate::modules::lib::functions::expand_paths::expand_paths;
use crate::modules::lib::LibError;

#[derive(Debug)]
pub struct LibIndex {
    pub lib_files: Vec<PathBuf>,
}

impl LibIndex {
    pub fn expand(lib_paths: Vec<String>) -> Result<Self, LibError> {
        let expanded_paths = expand_paths(lib_paths);
        let mut lib_files = Vec::new();

        for path in expanded_paths.iter() {
            let mut result = Self::recursive_read_dir(path)?;
            lib_files.append(&mut result);
        }

        Ok(Self {
            lib_files
        })
    }

    pub fn recursive_read_dir<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, LibError> {
        let dir_read = read_dir(path)?;
        let mut files = Vec::new();

        for entry in dir_read {
            let entry = entry?;

            let file_type = entry.file_type()?;

            if file_type.is_file() {
                files.push(entry.path());
            }

            if file_type.is_dir() {
                let mut result = Self::recursive_read_dir(entry.path())?;
                files.append(&mut result);
            }
        }

        Ok(files)
    }

    pub fn find_match_for<P: AsRef<Path>>(&self, lib_path: P) -> Result<Option<PathBuf>, LibError> {
        self.find_match_by_filename(lib_path)
    }

    pub fn find_match_by_filename<P: AsRef<Path>>(&self, lib_path: P) -> Result<Option<PathBuf>, LibError> {
        let lib_file_name = lib_path.as_ref()
            .file_name()
            .ok_or(LibError::MissingFileName)?;

        for lib_file in self.lib_files.iter() {
            let file_name = lib_file.file_name()
                .ok_or(LibError::MissingFileName)?;

            if file_name == lib_file_name {
                return Ok(Some(lib_file.clone()));
            }
        }

        Ok(None)
    }
}
