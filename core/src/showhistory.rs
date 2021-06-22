use colored::*;
use csv::ReaderBuilder;
use pager::Pager;
use std::{error::Error, fs::File, path::Path};

use crate::exception::Error::FileNotFoundError;

pub fn show_history(cache_dir: &Path, use_pager: bool) -> Result<(), Box<dyn Error>> {
    let cache_file = cache_dir.join("history_hash");
    println!("{}", "".normal());

    if cache_file.exists() {
        let _file = File::open(cache_file)?;
        if use_pager {
            Pager::new().setup();
        }
        load_history(_file)?;
        Ok(())
    } else {
        Err(Box::new(FileNotFoundError(
            "cache file is not found.alloc".to_string(),
        )))
    }
}

fn load_history(file: File) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);
    for result in rdr.records() {
        let record = result?;
        let date = &record[0];
        let hash = &record[1];
        let from_remote = &record[2];

        print!("{}", format!("commit {}", hash).yellow());
        match from_remote {
            "true" => print!(" ({})", "from remote".red()),
            _ => {}
        }
        print!("\n");
        print!("\t{}", format!("Date: {}", date).green());
        print!("\n\n")
    }
    Ok(())
}
