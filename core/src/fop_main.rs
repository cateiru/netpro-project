mod exception;
mod fileopration;

use fileopration::FileOperation;
use std::error::Error;
use std::{path::Path, thread, time};

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("README.md");
    let directory = Path::new(".cache");
    let fp = FileOperation::new(path, directory).unwrap();
    let sleep_time = time::Duration::from_nanos(100000000);
    let mut update = false;

    loop {
        update = fp.check(&update).unwrap();
        thread::sleep(sleep_time);
    }
}
