use std::{error::Error, fs::copy, path::Path};

use crate::exception::Error::FileNotFoundError;

/// Update cache to target.
///
/// Arguments:
/// - hash: hash value.
/// - target: target file path.
/// - cache_dir: cache directory.
pub fn apply_history(hash: String, target: &Path, cache_dir: &Path) -> Result<(), Box<dyn Error>> {
    let history_dir = cache_dir.join("history");
    let history_file = history_dir.join(hash);

    if history_file.exists() {
        copy(history_file, target)?;
        Ok(())
    } else {
        Err(Box::new(FileNotFoundError(
            "cache file is not found.".to_string(),
        )))
    }
}
