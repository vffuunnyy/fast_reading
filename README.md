# Fast Reading

A fast reading file library for Python

[![Python ≥3.9](https://img.shields.io/badge/Python-%3E%3D3.9-blue)](https://www.python.org/downloads/) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Description

Fast Reading is a high-performance file reading library for Python, written in Rust using [pyo3](https://github.com/PyO3/pyo3). It provides an efficient mechanism for reading files by loading them in batches and yielding file content one at a time. This makes it ideal for processing large volumes of data with minimal overhead.

## Features

- **High Performance:** Built with Rust for speedy file operations.
- **Seamless Python Integration:** Uses pyo3 for a natural Python interface.
- **Batch File Reading:** Reads files in batches to optimize I/O, then iterates over individual file data.
- **Flexible Iterators:** Provides different iterator types for various use cases.
- **Cross-Platform Compatibility:** Works with Python 3.9 and above.

## Installation

Ensure you have [maturin](https://github.com/PyO3/maturin) installed:

```shell
pip install fast-reading
```

# Tests

```shell
> 100_000 files of 1024 bytes each

$ Python reading time: 0.777500 seconds
$ FilesBatchIterator reading time (batch_size=5): 0.449203 seconds
$ FlattenFilesBatchIterator reading time (batch_size=5): 0.448566 seconds
```

```shell
> 100_000 files of 32768 bytes each

$ Python reading time: 1.138894 seconds
$ FilesBatchIterator reading time (batch_size=5): 0.847089 seconds
$ FlattenFilesBatchIterator reading time (batch_size=5): 0.835050 seconds
```

```shell
> 1_000_000 files of 4096 bytes each

$ Python reading time: 14.950660 seconds
$ FilesBatchIterator reading time (batch_size=5): 5.086445 seconds
$ FlattenFilesBatchIterator reading time (batch_size=5): 5.068229 seconds
```

# Usage
### FilesBatchIterator Example
This iterator reads files in batches and returns a list of file contents for each batch.

```python
from fast_reading import FilesBatchIterator

# Initialize the iterator for reading files from a directory with a batch size of 100
for file_bytes in FilesBatchIterator("/path/to/directory", batch_size=100):
    # file_bytes is a list of bytes read from one or more files
    print(file_bytes)
```

### FlattenFilesBatchIterator Example
This iterator loads a batch of files but yields file content one by one.

```python
from fast_reading import FlattenFilesBatchIterator

# Initialize the iterator to read files in batches of 5,
# but return one file's content at a time.
for file_content in FlattenFilesBatchIterator("/path/to/directory", batch_size=5):
    # file_content is the bytes content of a single file
    print(file_content)
```

# License
This project is licensed under the MIT License.

# Contact
Author: @vffuunnyy