use core::str;
use core::ptr::{null_mut};

use windows_sys::Win32::Foundation::{BOOL};
use windows_sys::Win32::Storage::FileSystem::WriteFile;
use windows_sys::Win32::System::Console::{GetStdHandle, STD_OUTPUT_HANDLE};

pub fn write_stdout(text: &str) -> BOOL {

    unsafe {
        let mut written = 0;

        WriteFile(
            GetStdHandle(STD_OUTPUT_HANDLE),
            text.as_ptr() as *const core::ffi::c_void,
            text.len() as u32,
            &mut written,
            null_mut()
        )
    }
}


