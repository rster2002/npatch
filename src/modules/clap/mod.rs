use std::path::PathBuf;
use clap::Parser;

/// Patch binaries to use specific dependencies.
#[derive(Debug, Parser)]
pub struct ClapRoot {
    /// Path to binary to patch.
    #[clap(short, long)]
    pub bin: Vec<PathBuf>,

    /// Directory of binaries to patch.
    #[clap(short = 'd', long)]
    pub bin_dir: Vec<PathBuf>,

    /// Paths of directories to search for dependencies, delimited by a colon.
    #[clap(short, long)]
    pub lib_paths: Vec<String>,

    /// Fail if a library cannot be found.
    #[clap(long)]
    pub fail_on_missing: bool,
}