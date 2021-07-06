use crate::exception::Error::FileNotFoundError;
use chrono::{DateTime, Local};
use csv::{Writer, WriterBuilder};
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    fs::{copy, create_dir_all, File, OpenOptions},
    io,
    io::{Read, Write},
    path::Path,
    time::SystemTime,
};

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
    pub fn check(&self, old: &bool) -> Result<bool, Box<dyn Error>> {
        let cache_file = self.cache_dir.join("cache");
        let is_exist_cache = cache_file.exists();
        let _hash = hash(&mut File::open(self.path)?, false)?;
        let cache_file_path = self.cache_dir.join("cache_file");

        let mut is_change = old.clone();

        if is_exist_cache {
            let buf = read(&cache_file)?;
            let current_hash = String::from_utf8(buf.clone())?;
            let cache_hash = hash(&mut File::open(&cache_file_path)?, false)?;

            if diff(&cache_hash, &current_hash)? {
                // write cache file to target file
                // external update.
                let text = read(&cache_file_path)?;
                write(&self.path, &text)?;
                write(&cache_file, &cache_hash.into_bytes())?;
                save_history(&cache_file_path, &self.cache_dir, true)?;
            } else if diff(&current_hash, &_hash)? {
                // internal cache update.
                copy_file(&self.path, &cache_file_path)?;
                write(&cache_file, &_hash.into_bytes())?;
                save_history(&self.path, &self.cache_dir, false)?;

                is_change = !is_change;
            }
        } else {
            // create initialize hash dir
            copy_file(&self.path, &cache_file_path)?;
            write(&cache_file, &_hash.into_bytes())?;
            save_history(&self.path, &self.cache_dir, false)?;

            is_change = !is_change;
        }
        Ok(change_flag(
            &self.cache_dir.join("is_update"),
            is_change,
            *old,
        )?)
    }
}

fn change_flag(
    cache_file: &Path,
    is_change: bool,
    old_change: bool,
) -> Result<bool, Box<dyn Error>> {
    if is_change ^ old_change {
        write(cache_file, &format!("{}", is_change).into_bytes())?;
        Ok(is_change)
    } else {
        Ok(old_change)
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
        hasher.update(format!(
            "{}",
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs()
        ));
    }
    let hash = &format!("{:x}", hasher.finalize())[..10];
    Ok(hash.to_string())
}

/// Copy file.
/// path to cache_dir/[name]
///
/// Arguments:
/// - path: target file path.
/// - to_path: copied file path.
///
fn copy_file(path: &Path, to_path: &Path) -> Result<(), Box<dyn Error>> {
    copy(path, to_path)?;
    return Ok(());
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

/// save history.
///
/// Arguments:
/// - file: target file path
/// - save_dir: save cache dir path.
fn save_history(file: &Path, save_dir: &Path, from_remote: bool) -> Result<(), Box<dyn Error>> {
    let local_datetime: DateTime<Local> = Local::now();
    let history_hash_path = save_dir.join("history_hash");
    let _hash = hash(&mut File::open(file)?, true)?;

    let mut wtr = match history_hash_path.exists() {
        true => {
            let _file = OpenOptions::new().append(true).open(history_hash_path)?;
            WriterBuilder::new().has_headers(false).from_writer(_file)
        }
        false => Writer::from_path(history_hash_path)?,
    };
    wtr.write_record(&[
        format!("{}", local_datetime),
        _hash.to_string(),
        format!("{}", from_remote),
    ])?;
    wtr.flush()?;

    let file_path = save_dir.join("history").join(_hash);
    copy(file, file_path)?;
    Ok(())
}
