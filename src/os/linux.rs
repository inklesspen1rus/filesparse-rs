use std::{fs::File, os::fd::AsRawFd, io::Error};

use libc::{fallocate64, FALLOC_FL_PUNCH_HOLE};

use crate::FileSparse;

impl FileSparse for File {
    fn enable_file_sparse(&self) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn deallocate_region(&self, region: (u64, u64)) -> Result<(), std::io::Error> {
        assert!(region.1 > region.0 && region.1 < i64::MAX as _);
        let ret = unsafe { fallocate64(self.as_raw_fd(), FALLOC_FL_PUNCH_HOLE, (region.0) as _, (region.1 - region.0) as _) };

        match ret {
            0 => Ok(()),
            x => Err(Error::from_raw_os_error(x))
        }
    }
}