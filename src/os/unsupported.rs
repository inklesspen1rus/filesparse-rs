use crate::FileSparse;
use std::fs::File;

impl FileSparse for File {
    const SPARSE_SUPPORTED: bool = false;
}
