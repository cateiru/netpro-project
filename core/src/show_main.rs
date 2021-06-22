mod showhistory;
mod exception;

use std::error::Error;
use std::path::Path;
use showhistory::show_history;


fn main() -> Result<(), Box<dyn Error>> {
    show_history(Path::new(".cache"), true)?;
    Ok(())
}
