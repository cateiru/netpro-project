use blake2::{Blake2b, Digest};
use std::{error::Error, fs::File, io, io::BufReader, path::Path};

pub struct FileOperation<'a> {
    path: &'a Path,
    cache_path: &'a Path,
}

impl<'a> FileOperation<'a> {
    pub fn new(path: &'a Path, cache_path: &'a Path) -> Option<Self> {
        match path.exists() && path.is_file() && cache_path.is_dir() {
            true => Some(Self {
                path: path,
                cache_path: cache_path,
            }),
            _ => None,
        }
    }

    pub fn check(&self) -> Result<bool, Box<dyn Error>> {
        let is_exist_cache = self.cache_path.exists();

        if is_exist_cache {
        }else {
        }

        Ok(true)
    }

    fn get_latest_cache(&self, cache: File) -> Result<String, Box<dyn Error>> {
        let latest_hash = BufReader<File>::new(cache).lines();

        Ok(format!("{}", latest_hash.next()))
    }

    fn diff(&self, file: File, cache: String) -> Result<bool, Box<dyn Error>> {
        let current_hash = self.hash(&file)?;

        match &*current_hash {
            hash => Ok(true),
            _ => Ok(false),
        }
    }

    fn hash(&self, file: &File) -> Result<String, Box<dyn Error>>  {
        let hasher = Blake2b::new();
        let _ = io::copy(&mut file, &mut hasher)?;
        format!("{:x}", hasher.finalize())
    }

    fn file_open(&self) -> Result<File, Box<dyn Error>> {
        Ok(File::open(self.path)?)
    }

    fn cache_read(&self) -> Result<File, Box<dyn Error>> {
        Ok(File::open(self.cache_path)?)
    }

    fn cache_create(&self) -> Result<File, Box<dyn Error>> {
        Ok(File::create(self.cache_path)?)
    }
}
