pub use std::io::{Error, ErrorKind};
pub mod os;

/// Generic set of methods to work with sparsed files.
/// This trait is implemented for [std::fs::File] at any target.
/// 
/// # Supported OS
/// + Windows
/// + Linux
/// + Android
/// 
/// On any other platform API is no-op with Unsupported error.
/// 
/// # Platform-specific
/// 
/// On windows, all methods use blocking calls.
/// 
/// On unix, file must be opened in blocking mode.
pub trait FileSparse {
    /// Compile-time indicator if current target has support
    const SPARSE_SUPPORTED: bool = true;

    /// Ensures file is able to have dellocated regions.
    /// This method may return falsy-positive result.
    /// 
    /// + Windows: set sparse flag to file.
    /// + Unix: no-op with always success.
    fn enable_file_sparse(&self) -> Result<(), Error> {
        Err(ErrorKind::Unsupported.into())
    }

    /// Deallocate file region on disk.
    fn deallocate_region(&self, #[allow(unused)] region: (u64, u64)) -> Result<(), Error> {
        Err(ErrorKind::Unsupported.into())
    }

    /// Find next allocated region starting from specified offset.
    /// Currently, it's unsupported at any platform.
    fn find_next_data(&self, #[allow(unused)] start_from: u64) -> Result<u64, Error> {
        Err(ErrorKind::Unsupported.into())
    }
    
    /// Find next non-allocated region starting from specified offset.
    /// Currently, it's unsupported at any platform.
    fn find_next_hole(&self, #[allow(unused)] start_from: u64) -> Result<u64, Error> {
        Err(ErrorKind::Unsupported.into())
    }
}