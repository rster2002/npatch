use std::env;
use clap::Parser;
use crate::modules::clap::ClapRoot;
use crate::modules::lib::{LibIndex};
use crate::modules::patch::{patch_binary, patch_dir};

mod modules;

fn main() {
    let mut options = ClapRoot::parse();

    if let Ok(ld_library_path) = env::var("LD_LIBRARY_PATH") {
        options.lib_paths.push(ld_library_path);
    }

    let lib_index = match LibIndex::expand(options.lib_paths) {
        Ok(index) => index,
        Err(e) => panic!("Failed to expand lib paths: {}", e)
    };

    for bin in &options.bin {
        match patch_binary(bin, &lib_index, options.fail_on_missing) {
            Ok(_) => println!("Patched {}", bin.display()),
            Err(e) => println!("Failed to patch {}: {}", bin.display(), e)
        }
    }

    for dir in &options.bin_dir {
        match patch_dir(dir, &lib_index, options.fail_on_missing) {
            Ok(_) => println!("Patched dir {}", dir.display()),
            Err(e) => println!("Failed to patch dir {}: {}", dir.display(), e)
        }
    }
}
