use std::path::PathBuf;

pub fn expand_paths(paths: Vec<String>) -> Vec<PathBuf> {
    paths.into_iter()
        .map(|path| {
            path.split(':')
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .flatten()
        .map(PathBuf::from)
        .collect()
}