use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::fs;
use std::path::{Path, PathBuf};

#[pyclass]
struct FileData {
    path: PathBuf,
    content: PyObject,
}

#[pyclass]
struct FilesBatchIterator {
    batch_size: usize,
    current_index: usize,
    files: Vec<PathBuf>,
}

#[pyclass]
struct FlattenFilesIterator {
    current_index: usize,
    files: Vec<PathBuf>,
}

#[pymethods]
impl FileData {
    #[getter]
    fn path(&self) -> PyResult<String> {
        Ok(self.path.to_string_lossy().into_owned())
    }

    #[getter]
    fn content(&self, py: Python<'_>) -> PyResult<PyObject> {
        Ok(self.content.clone_ref(py))
    }

    fn __len__(&self, py: Python<'_>) -> usize {
        if let Ok(bytes) = self.content.extract::<Bound<'_, PyBytes>>(py) {
            bytes.len().expect("Failed to get length of bytes")
        } else {
            0
        }
    }
}

#[pymethods]
impl FilesBatchIterator {
    #[new]
    #[pyo3(signature = (directory, batch_size=1))]
    fn new(directory: &str, batch_size: Option<usize>) -> PyResult<Self> {
        let batch_size = batch_size.unwrap_or(1);
        let directory_path = Path::new(directory);
        let files: Vec<PathBuf> = fs::read_dir(directory_path)
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                    "Error reading directory {}: {}",
                    directory_path.display(),
                    e
                ))
            })?
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

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<Vec<FileData>>> {
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
                    batch.push(FileData {
                        path: file_path.to_path_buf(),
                        content: pybytes.into(),
                    });
                }
                Err(err) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                        "Error reading file {:?}: {}",
                        file_path, err
                    )));
                }
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
impl FlattenFilesIterator {
    #[new]
    #[pyo3(signature = (directory))]
    fn new(directory: &str) -> PyResult<Self> {
        let directory_path = Path::new(directory);
        let files: Vec<PathBuf> = fs::read_dir(directory_path)
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                    "Error reading directory {}: {}",
                    directory_path.display(),
                    e
                ))
            })?
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .collect();

        Ok(FlattenFilesIterator {
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

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<FileData>> {
        let py = slf.py();

        if slf.current_index >= slf.files.len() {
            return Ok(None);
        }

        let file_path = slf.files[slf.current_index].clone();
        slf.current_index += 1;

        match fs::read(file_path.clone()) {
            Ok(data) => {
                let pybytes = PyBytes::new(py, &data);
                Ok(Some(FileData {
                    path: file_path,
                    content: pybytes.into(),
                }))
            }
            Err(err) => Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Error reading file {:?}: {}",
                file_path, err
            ))),
        }
    }
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FilesBatchIterator>()?;
    m.add_class::<FlattenFilesIterator>()?;
    m.add_class::<FileData>()?;
    Ok(())
}
