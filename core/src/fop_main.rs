mod exception;
mod fileopration;
mod showhistory;

use std::error::Error;
use std::{path::Path, thread, time};
use fileopration::FileOperation;


fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("README.md");
    let directory = Path::new(".cache");
    let fp = FileOperation::new(path, directory).unwrap();
    let sleep_time = time::Duration::from_nanos(100000000);

    while !fp.check().unwrap() {
        thread::sleep(sleep_time);
    }
    Ok(())
}
