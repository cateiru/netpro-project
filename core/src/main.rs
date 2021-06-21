mod exception;
mod fileopration;

use std::error::Error;
use std::path::Path;
use fileopration::FileOperation;

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("README.md");
    let directory = Path::new(".cache");
    let fp = FileOperation::new(path, directory).unwrap();
    fp.check().unwrap();
    Ok(())
}
