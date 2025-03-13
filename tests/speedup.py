import os
import time
import shutil
import tempfile

from fast_reading import FilesBatchIterator, FlattenFilesIterator


def generate_random_files(directory: str, num_files: int, file_size: int) -> None:
    os.makedirs(directory, exist_ok=True)
    for i in range(num_files):
        file_path = os.path.join(directory, f"file_{i}.dat")
        with open(file_path, "wb") as f:
            f.write(os.urandom(file_size))


def benchmark_native_reading(directory: str) -> float:
    file_list = [os.path.join(directory, f) for f in os.listdir(directory)]
    start = time.perf_counter()
    total_len = 0
    for file_path in file_list:
        with open(file_path, "rb") as f:
            data = f.read()
            total_len += len(data)
    elapsed = time.perf_counter() - start
    print("$ Python reading time (total_len={}): {:.6f} seconds".format(total_len, elapsed))
    return elapsed


def benchmark_files_batch_iterator(directory: str, batch_size: int) -> float:
    start = time.perf_counter()
    iterator = FilesBatchIterator(directory, batch_size)
    total_len = 0
    for batch in iterator:
        for data in batch:
            total_len += len(data)
    elapsed = time.perf_counter() - start
    print(
        "$ FilesBatchIterator reading time (total_len={}, batch_size={}): {:.6f} seconds".format(
            total_len, batch_size, elapsed
        )
    )
    return elapsed


def benchmark_flatten_files_batch_iterator(directory: str) -> float:
    start = time.perf_counter()
    iterator = FlattenFilesIterator(directory)
    total_len = 0
    for data in iterator:
        total_len += len(data)
    elapsed = time.perf_counter() - start
    print(
        "$ FlattenFilesBatchIterator reading time (total_len={}): {:.6f} seconds".format(total_len, elapsed)
    )
    return elapsed


if __name__ == "__main__":
    test_dir = os.path.join(tempfile.gettempdir(), "fast_reading_test_files")
    num_files = 1_000_000
    file_size = 2 ** 12

    if os.path.exists(test_dir):
        shutil.rmtree(test_dir)

    print(
        "> Generating {} files of {} bytes each in '{}'".format(
            num_files, file_size, test_dir
        )
    )
    generate_random_files(test_dir, num_files, file_size)

    benchmark_native_reading(test_dir)
    benchmark_files_batch_iterator(test_dir, batch_size=5)
    benchmark_flatten_files_batch_iterator(test_dir)

    shutil.rmtree(test_dir)
