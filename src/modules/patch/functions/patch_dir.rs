use std::fs::read_dir;
use std::path::Path;
use crate::modules::lib::LibIndex;
use crate::modules::patch::error::PatchError;
use crate::modules::patch::patch_binary;

pub fn patch_dir<P: AsRef<Path>>(
    path: P,
    lib_index: &LibIndex,
    fail_on_missing: bool,
) -> Result<(), PatchError> {
    for entry in read_dir(path)? {
        let entry = entry?;
        patch_binary(entry.path(), lib_index, fail_on_missing)?;
    }

    Ok(())
}