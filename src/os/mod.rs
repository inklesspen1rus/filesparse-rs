#[cfg(any(target_os = "linux", target_os = "android"))]
pub mod linux;
#[cfg(windows)]
pub mod windows;

#[cfg(not(any(windows, any(target_os = "linux", target_os = "android"))))]
pub mod unsupported;
