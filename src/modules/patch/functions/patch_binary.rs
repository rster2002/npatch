use std::path::{Path, PathBuf};
use std::process::Command;
use crate::modules::lib::{dependencies_of, LibIndex};
use crate::modules::patch::error::PatchError;

pub fn patch_binary<P: AsRef<Path>>(binary_path: P, lib_index: &LibIndex) -> Result<(), PatchError> {
    if cfg!(target_os = "macos") {
        install_name_tool_patch_binary(binary_path, lib_index)
    } else {
        unimplemented!()
    }
}

fn install_name_tool_patch_binary<P: AsRef<Path>>(binary_path: P, lib_index: &LibIndex) -> Result<(), PatchError> {
    println!("Patching {}", binary_path.as_ref().display());

    let mut command = Command::new("install_name_tool");

    let dependencies = dependencies_of(&binary_path)?;

    for dependency in dependencies.iter() {
        let lib_file = lib_index.find_match_for(dependency)?;

        let Some(lib_file) = lib_file else {
            println!("Could not find lib file for dependency {}", dependency.display());
            // TODO handle fail
            continue;
        };

        command.arg("-change").arg(dependency).arg(lib_file);
    }

    let path_string = binary_path.as_ref()
        .to_str()
        .ok_or(PatchError::FailedToRepresentPath)?;

    command.arg(path_string);

    let output = command.output()?;

    if !output.status.success() {
        let output_string = String::from_utf8(output.stderr)?;
        return Err(PatchError::FailedToPatchInstallNameTool(output.status.code().unwrap_or(100), output_string));
    }

    Ok(())
}