# C++ to Rust to C++ FFI Demo
 * Use `bindgen` and `cbindgen` to generate boilerplate code.
 * Use `cmake` for full build

## Usage
To generate bindings:
```sh
$ cargo build
```

To build everything:
```sh
$ mkdir build && cd build
$ cmake ..
$ make
