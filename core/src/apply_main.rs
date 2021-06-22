mod applyhistory;
mod exception;

use applyhistory::apply_history;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let hash = "62bc7aa6fd".to_string();
    let target = Path::new("README.md");
    let dir = Path::new(".cache");
    apply_history(hash, target, dir).unwrap();
    Ok(())
}
