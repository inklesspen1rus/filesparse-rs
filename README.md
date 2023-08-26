# filesparse-rs
Rust library provides API for sparsed files at several platforms aims to compile at any platform

# Usage

Library declares [FileSparse](src/lib.rs) trait to add methods for standard std::io::File type. Here is example for it's usage:

```rs
use std::env::args;
use std::fs::File;
use std::path::Path;

// To use enable_file_sparse and deallocate_region methods
use filesparse::FileSparse;

fn main() {
    let path = args()
        .skip(1)
        .next()
        .unwrap();
    let path = Path::new(&path);

    // Open file with write support
    let file = File::options()
        .write(true)
        .open(path)
        .unwrap();

    // Get size of file
    let length = file.metadata().unwrap().len();

    // Prepare file to be sparsed
    // NO-OP on Linux but required for Windows
    file.enable_file_sparse().unwrap();

    // Deallocate all file
    file.deallocate_region((0, length)).unwrap();
}
```

# Supported OS
+ Windows
+ Linux
+ Android

You can help with supporting other OS =\)
