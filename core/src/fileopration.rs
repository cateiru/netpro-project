use std::{error::Error, fs::File, path::Path};

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

    fn cache_file_open(&self) -> Result<File, Box<dyn Error>> {
        let file = match self.cache_path.exists() {
            true => File::open(self.cache_path)?,
            false => File::create(self.cache_path)?,
        };

        Ok(file)
    }
}
