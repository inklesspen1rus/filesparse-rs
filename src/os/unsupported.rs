use std::fs::File;
use crate::FileSparse;

impl FileSparse for File {
    const SPARSE_SUPPORTED: bool = false;
}