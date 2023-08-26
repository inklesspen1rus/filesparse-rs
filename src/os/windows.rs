use std::fs::File;
use std::io::Error;
use std::os::windows::prelude::{RawHandle, AsRawHandle};
use winapi::shared::minwindef::DWORD;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::ioapiset::DeviceIoControl;
use winapi::um::winioctl::{FSCTL_SET_ZERO_DATA, FSCTL_SET_SPARSE};
use winapi::um::winnt::LARGE_INTEGER;

use crate::FileSparse;

impl FileSparse for File {
    fn deallocate_region(&self, region: (u64, u64)) -> Result<(), std::io::Error> {
        match unsafe { file_sparse_region(self.as_raw_handle(), region) } {
            Ok(()) => Ok(()),
            Err(ecode) => {
                Err(Error::from_raw_os_error(ecode as _))
            }
        }
    }

    fn enable_file_sparse(&self) -> Result<(), std::io::Error> {
        match unsafe { file_set_sparse(self.as_raw_handle(), true) } {
            Ok(()) => Ok(()),
            Err(ecode) => {
                Err(Error::from_raw_os_error(ecode as _))
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
struct FileSetSparseBuffer {
    set_sparse: bool
}

#[repr(C)]
struct FileZeroDataInformation {
    file_offset: LARGE_INTEGER,
    beyond_final_zero: LARGE_INTEGER,
}

// Safety: Region is valid
pub unsafe fn file_sparse_region(hndl: RawHandle, region: (u64, u64)) -> Result<(), DWORD> {
    assert!(region.0 <= i64::MAX as u64);
    assert!(region.1 <= i64::MAX as u64);

    let mut sparse = core::mem::MaybeUninit::<FileZeroDataInformation>::uninit();
    *(*sparse.as_mut_ptr()).file_offset.QuadPart_mut() = region.0 as i64;
    *(*sparse.as_mut_ptr()).beyond_final_zero.QuadPart_mut() = region.1 as i64;
    let mut sparse = sparse.assume_init();

    let mut unknown = 0u32;

    let res = DeviceIoControl(
        hndl as _,
        FSCTL_SET_ZERO_DATA,
        &mut sparse as *mut FileZeroDataInformation as _,
        core::mem::size_of::<FileZeroDataInformation>() as u32,
        core::ptr::null_mut(),
        0,
        &mut unknown as *mut u32,
        core::ptr::null_mut()
    );

    if res == 0 {
        Err(GetLastError())
    } else {
        Ok(())
    }
}

// value == true Safety
// value == false Safety: file must not have sparsed regions && not supported Windows Server 2003 and Windows XP and possibly doesn't work at all
// https://learn.microsoft.com/en-us/windows/win32/api/winioctl/ni-winioctl-fsctl_set_sparse
// http://www.vsokovikov.narod.ru/New_MSDN_API/Menage_files/fsctl_set_sparse.htm
pub unsafe fn file_set_sparse(hndl: RawHandle, value: bool) -> Result<(), DWORD> {
    let mut sparse = FileSetSparseBuffer {
        set_sparse: value
    };

    let mut unknown = 0u32;

    let res = DeviceIoControl(
        hndl as _,
        FSCTL_SET_SPARSE,
        &mut sparse as *mut FileSetSparseBuffer as _,
        core::mem::size_of::<FileSetSparseBuffer>() as u32,
        core::ptr::null_mut(),
        0,
        &mut unknown as *mut u32,
        core::ptr::null_mut()
    );

    if res == 0 {
        Err(GetLastError())
    } else {
        Ok(())
    }
}