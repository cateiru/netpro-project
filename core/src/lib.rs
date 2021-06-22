mod exception;
mod fileopration;
mod showhistory;

pub use fileopration::FileOperation;
pub use showhistory::show_history;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::{path::Path, thread, time};
use ctrlc;


#[pyfunction]
fn fop(file_path: String, dir: String) -> PyResult<()> {
    ctrlc::set_handler(|| std::process::exit(2)).unwrap();
    let _path = Path::new(&file_path);
    let directory = Path::new(&dir);
    let fp = FileOperation::new(_path, directory).unwrap();
    let sleep_time = time::Duration::from_nanos(100000000);

    while !fp.check().unwrap() {
        thread::sleep(sleep_time);
    }
    Ok(())
}

#[pyfunction]
fn show(dir: String, use_pager: bool) -> PyResult<()> {
    ctrlc::set_handler(|| std::process::exit(2)).unwrap();
    let cache_dir = Path::new(&dir);
    show_history(cache_dir, use_pager).unwrap();
    Ok(())
}

#[pymodule]
fn core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fop, m)?)?;
    m.add_function(wrap_pyfunction!(show, m)?)?;

    Ok(())
}
