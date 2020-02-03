# Rust Python Modules
This project demonstrates how rust can be used to build python modules.
Python modules are built using [Py03] rust bindings. Currently Py03 requires Python 3.5 to build.

## Building
Install rust nightly.
```bash
rustup install nightly
``` 
Build the project within the `rust` folder.
```bash
cd rust
cargo +nightly build --release
```
This will create a dll file on windows,
 a dylib file on OSX, or a .so file on Linux in `rust/target/release`.
 Copy the file `wandell_rust.[dll,dylib,so]` into the python 
 interpreter `site-packages` folder. Rename this file to `wandell_rust.pyd`. 
 
1. quick_sort
   * A quick sort algorithm demo is used to show that rust can run things very fast. 
   The python file `test_quick_sort_vs_python_sorted.py` is provided to show that the rust custom 
   quick sort code will sort a list of 100 random numbers faster than python's `sorted` method.
   This is a contrived example simply showing that for a completely random list, a custom quick sort
   implemented in a fast language can outperform pythons built in `timsort` based functions.

[Py03]:https://github.com/PyO3/pyo3