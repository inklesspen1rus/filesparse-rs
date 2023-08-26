#[cfg(windows)] pub mod windows;
#[cfg(any(target_os = "linux", target_os = "android"))] pub mod linux;

#[cfg(not(any(windows, any(target_os = "linux", target_os = "android"))))]
pub mod unsupported;
