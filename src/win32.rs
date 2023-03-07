use core::ffi::c_void;
use core::str;
use core::ptr::{null_mut};

use windows_sys::Win32::Foundation::{BOOL, GetLastError, WIN32_ERROR};
use windows_sys::Win32::Storage::FileSystem::WriteFile;
use windows_sys::Win32::System::Console::{GetStdHandle, STD_OUTPUT_HANDLE, WriteConsoleW};
use crate::winstr::WinStr;

pub fn write_stdout(text: &str) -> BOOL {

    unsafe {
        let mut written = 0;

        WriteFile(
            GetStdHandle(STD_OUTPUT_HANDLE),
            text.as_ptr() as *const c_void,
            text.len() as u32,
            &mut written,
            null_mut()
        )
    }
}

pub fn write_console(text : &WinStr) -> Result<(),WIN32_ERROR> {
    unsafe {
        let mut written : u32 = 0;
        match
            WriteConsoleW(
            GetStdHandle(STD_OUTPUT_HANDLE),
            text.as_ptr() as * const c_void,
            text.len() as u32,
            &mut written,
            null_mut() ) {
            0 => { Err(GetLastError()) },
            _ => Ok(())
        }
    }
}

const WIN_CRLF: WinStr = WinStr( &[ 13, 10, 0 ] );

pub fn writeln_console(text : &WinStr) -> Result<(),WIN32_ERROR> {
    write_console(text)?;
    write_console(&WIN_CRLF)?;
    Ok(())
}


