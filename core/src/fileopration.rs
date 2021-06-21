use crate::exception::Error::{CannotCopyError, FileNotFoundError};
use sha2::{Sha256, Digest};
use std::{
    error::Error,
    fs::{copy, create_dir_all, File, OpenOptions},
    io,
    io::{Read, Write},
    path::Path,
    time::SystemTime
};
use csv::{WriterBuilder, Writer};
use chrono::{Local, DateTime};

pub struct FileOperation<'a> {
    path: &'a Path,
    cache_dir: &'a Path,
}

impl<'a> FileOperation<'a> {
    pub fn new(path: &'a Path, cache_dir: &'a Path) -> Result<Self, Box<dyn Error>> {
        match path.exists() && path.is_file() {
            true => {
                let history_dir = cache_dir.join("history");
                if !history_dir.exists() {
                    create_dir_all(history_dir)?;
                }
                Ok(Self {
                    path: path,
                    cache_dir: cache_dir,
                })
            }
            _ => Err(Box::new(FileNotFoundError(
                "path is not found or not file.".to_string(),
            ))),
        }
    }

    /// Check file update.
    ///
    /// Returns:
    /// - bool: True if updated, false otherwise.
    pub fn check(&self) -> Result<bool, Box<dyn Error>> {
        let cache_file = self.cache_dir.join("cache");
        let is_exist_cache = cache_file.exists();
        let _hash = hash(&mut File::open(self.path)?, false)?;

        let mut is_change = false;

        if is_exist_cache {
            let buf = read(&cache_file)?;
            let current_hash = String::from_utf8(buf.clone())?;

            if let Some(file_name) = self.path.file_name() {
                let cache_file_path = self.cache_dir.join(file_name);
                let cache_hash = hash(&mut File::open(&cache_file_path)?, false)?;

                if diff(&cache_hash, &current_hash)? {
                    // write cache file to target file
                    let text = read(&cache_file_path)?;
                    write(&self.path, &text)?;

                    // save history
                    let history_hash = hash(&mut File::open(&cache_file_path)?, true)?;
                    save_history(&history_hash, &cache_file_path, &self.cache_dir)?;

                } else if diff(&current_hash, &_hash)? {
                    copy_file(&self.path, &self.cache_dir)?;
                    write(&cache_file, &_hash.into_bytes())?;

                    // save history
                    let history_hash = hash(&mut File::open(self.path)?, true)?;
                    save_history(&history_hash, &self.path, &self.cache_dir)?;

                    is_change = true;
                }
            }
        } else {
            // create initialize hash dir
            copy_file(&self.path, &self.cache_dir)?;
            write(&cache_file, &_hash.into_bytes())?;

            // save history
            let history_hash = hash(&mut File::open(self.path)?, true)?;
            save_history(&history_hash, &self.path, &self.cache_dir)?;

            is_change = true;
        }

        Ok(is_change)
    }
}

/// Compare hashes.
///
/// Arguments:
/// - current_hash: hash.
/// - _hash: target hash.
///
/// Returns;
/// - bool: True if the hashes are the same.
fn diff(current_hash: &String, _hash: &String) -> Result<bool, Box<dyn Error>> {
    // println!("{}\n{}", current_hash, _hash);

    if *_hash == *current_hash {
        return Ok(false);
    }
    Ok(true)
}

/// Create file to hash.
///
/// Arguments:
/// - file: file data.
///
/// Returns:
/// - String: hash.
///
fn hash(mut file: &mut File, use_time: bool) -> Result<String, Box<dyn Error>> {
    let mut hasher = Sha256::new();
    let _ = io::copy(&mut file, &mut hasher)?;
    if use_time {
        hasher.update(format!("{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs()));
    }
    let hash = &format!("{:x}", hasher.finalize())[..10];
    Ok(hash.to_string())
}

/// Copy file.
/// path to cache_dir/[name]
///
/// Arguments:
/// - path: target file path.
/// - cache_dir: copied file path.
///
fn copy_file(path: &Path, cache_dir: &Path) -> Result<(), Box<dyn Error>> {
    if let Some(file_name) = path.file_name() {
        let to_path = cache_dir.join(file_name);
        copy(path, to_path)?;
        return Ok(());
    }
    Err(Box::new(CannotCopyError()))
}

/// Read file.
///
/// Arguments:
/// - path: file path.
///
/// Returns:
/// - vec![u8]: text value.
fn read(path: &Path) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;

    Ok(buf)
}

/// Write file.
///
/// Arguments:
/// - path: file path.
/// - text: write text.
fn write(path: &Path, text: &Vec<u8>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    file.write_all(text)?;
    Ok(())
}

fn save_history(hash: &String, file: &Path, save_dir: &Path) -> Result<(), Box<dyn Error>> {
    let local_datetime: DateTime<Local> = Local::now();
    let history_hash_path = save_dir.join("history_hash");

    let mut wtr = match history_hash_path.exists() {
        true => {
            let _file = OpenOptions::new().append(true).open(history_hash_path)?;
            WriterBuilder::new().has_headers(false).from_writer(_file)
        },
        false => Writer::from_path(history_hash_path)?,
    };
    wtr.write_record(&[format!("{}", local_datetime), hash.to_string()])?;
    wtr.flush()?;

    let file_path = save_dir.join("history").join(hash);
    copy(file, file_path)?;
    Ok(())
}
