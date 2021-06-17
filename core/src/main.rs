use repo_sync::FileOperation;
use std::{error::Error, fs::File, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let target = Path::new("sample.md");
    let cache = Path::new(".cache");

    if !target.exists() {
        create_file(target);
    }

    let file_op = FileOperation::new(&target, &cache)?;
    let is_change = file_op.check()?;

    if is_change {
        println!("Change!!!");
    } else {
        println!("Not Change!!!");
    }

    Ok(())
}

fn create_file(path: &Path) {
    File::create(path).unwrap();
}
