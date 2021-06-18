mod exception;
mod fileopration;

use fileopration::FileOperation;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::path::Path;

#[pyfunction]
fn fop(file_path: String, dir: String) -> PyResult<()> {
    let _path = Path::new(&file_path);
    let directory = Path::new(&dir);
    let fp = FileOperation::new(_path, directory).unwrap();

    while !fp.check().unwrap() {}

    Ok(())
}

#[pymodule]
fn core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fop, m)?)?;

    Ok(())
}

// pub use exception::Error::{CannotCopyError, FileNotFoundError};
// pub use fileopration::FileOperation;
