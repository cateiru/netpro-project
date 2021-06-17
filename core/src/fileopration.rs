use crate::exception::Error::{CannotCopyError, FileNotFoundError};
use blake2::{Blake2b, Digest};
use std::{
    error::Error,
    fs::{copy, create_dir_all, File},
    io,
    io::{BufReader, Read, Write},
    path::Path,
};

pub struct FileOperation<'a> {
    path: &'a Path,
    cache_dir: &'a Path,
}

impl<'a> FileOperation<'a> {
    pub fn new(path: &'a Path, cache_dir: &'a Path) -> Result<Self, Box<dyn Error>> {
        match path.exists() && path.is_file() {
            true => {
                if !cache_dir.exists() {
                    create_dir_all(cache_dir)?;
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

    pub fn check(&self) -> Result<bool, Box<dyn Error>> {
        let cache_file = self.cache_dir.join("cache");
        let is_exist_cache = cache_file.exists();
        let hash = hash(&mut File::open(self.path)?)?;

        let mut is_change = false;

        if is_exist_cache {
            let buf = read(&cache_file)?;
            if diff(buf.clone(), &hash)? {
                copy_file(&self.path, &self.cache_dir)?;
                write(&cache_file, &hash.into_bytes())?;
                is_change = true;
            } else {
                write(&cache_file, &buf)?;
            }
        } else {
            copy_file(&self.path, &self.cache_dir)?;
            write(&cache_file, &hash.into_bytes())?;
            is_change = true;
        }

        Ok(is_change)
    }
}

fn diff(buf: Vec<u8>, _hash: &String) -> Result<bool, Box<dyn Error>> {
    let current_hash = String::from_utf8(buf)?;
    // println!("{}\n{}", current_hash, _hash);

    if *_hash == current_hash {
        return Ok(false);
    }
    Ok(true)
}

fn hash(mut file: &mut File) -> Result<String, Box<dyn Error>> {
    let mut hasher = Blake2b::new();
    let _ = io::copy(&mut file, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
}

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
/// - path(&Path): file path.
///
/// Returns:
/// - vec![u8]: text value.
fn read(path: &Path) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;

    Ok(buf)
}

/// Create file.
///
/// Arguments:
/// - path(&Path): file path.
/// - text(Vec<u8>).
fn write(path: &Path, text: &Vec<u8>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    file.write_all(text)?;
    Ok(())
}
