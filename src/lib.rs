use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};

#[pyclass]
struct FilesBatchIterator {
    batch_size: usize,
    current_index: usize,
    files: Vec<PathBuf>,
}

#[pyclass]
struct FlattenFilesBatchIterator {
    batch_size: usize,
    current_index: usize,
    files: Vec<PathBuf>,
    buffer: VecDeque<PyObject>,
}

#[pymethods]
impl FilesBatchIterator {
    #[new]
    #[pyo3(signature = (directory, batch_size=1))]
    fn new(directory: &str, batch_size: Option<usize>) -> PyResult<Self> {
        let batch_size = batch_size.unwrap_or(1);
        let directory_path = Path::new(directory);
        let files: Vec<PathBuf> = fs::read_dir(directory_path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(
                format!("Error reading directory {}: {}", directory_path.display(), e)
            ))?
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .collect();

        Ok(FilesBatchIterator {
            batch_size,
            current_index: 0,
            files,
        })
    }

    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __len__(&self) -> usize {
        self.files.len()
    }

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<Vec<PyObject>>> {
        let py = slf.py();
        if slf.current_index >= slf.files.len() {
            return Ok(None);
        }

        let mut batch = Vec::new();
        for _ in 0..slf.batch_size {
            if slf.current_index >= slf.files.len() {
                break;
            }
            let file_path = &slf.files[slf.current_index];
            match fs::read(file_path) {
                Ok(data) => {
                    let pybytes = PyBytes::new(py, &data);
                    batch.push(pybytes.into());
                },
                Err(err) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(
                        format!("Error reading file {:?}: {}", file_path, err)
                    ));
                },
            }
            slf.current_index += 1;
        }

        if batch.is_empty() {
            Ok(None)
        } else {
            Ok(Some(batch))
        }
    }
}

#[pymethods]
impl FlattenFilesBatchIterator {
    #[new]
    #[pyo3(signature = (directory, batch_size=1))]
    fn new(directory: &str, batch_size: Option<usize>) -> PyResult<Self> {
        let batch_size = batch_size.unwrap_or(1);
        let directory_path = Path::new(directory);
        let files: Vec<PathBuf> = fs::read_dir(directory_path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(
                format!("Error reading directory {}: {}", directory_path.display(), e)
            ))?
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .collect();

        Ok(FlattenFilesBatchIterator {
            batch_size,
            current_index: 0,
            files,
            buffer: VecDeque::new(),
        })
    }

    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __len__(&self) -> usize {
        self.files.len()
    }

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<PyObject>> {
        let py = slf.py();
        
        if slf.buffer.is_empty() {
            if slf.current_index >= slf.files.len() {
                return Ok(None);
            }
            for _ in 0..slf.batch_size {
                if slf.current_index >= slf.files.len() {
                    break;
                }
                let file_path = &slf.files[slf.current_index];
                match fs::read(file_path) {
                    Ok(data) => {
                        let pybytes = PyBytes::new(py, &data);
                        slf.buffer.push_back(pybytes.into());
                    },
                    Err(err) => {
                        return Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(
                            format!("Error reading file {:?}: {}", file_path, err)
                        ));
                    },
                }
                slf.current_index += 1;
            }
        }
        
        if let Some(item) = slf.buffer.pop_front() {
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FilesBatchIterator>()?;
    m.add_class::<FlattenFilesBatchIterator>()?;
    Ok(())
}
