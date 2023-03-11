use alloc::vec::Vec;
use core::ffi::c_void;
use core::str;
use core::ptr::{null_mut};

use windows_sys::Win32::Foundation::{BOOL, CloseHandle, GetLastError, HANDLE, INVALID_HANDLE_VALUE, WIN32_ERROR};
use windows_sys::Win32::Storage::FileSystem::{CreateFileW, FILE_ACCESS_FLAGS, FILE_FLAG_BACKUP_SEMANTICS, FILE_SHARE_READ, FILE_SHARE_WRITE, GetFinalPathNameByHandleW, OPEN_EXISTING, WriteFile};
use windows_sys::Win32::System::Console::{GetStdHandle, STD_OUTPUT_HANDLE, WriteConsoleW};
use windows_sys::Win32::System::WindowsProgramming::VOLUME_NAME_DOS;
use crate::wstr::WStr;

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

pub fn write_console(text : &WStr) -> Result<(),WIN32_ERROR> {
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

const WIN_CRLF: WStr = WStr( &[ 13, 10 ] );

pub fn writeln_console(text : &WStr) -> Result<(),WIN32_ERROR> {
    write_console(text)?;
    write_console(&WIN_CRLF)?;
    Ok(())
}

struct SafeHandle(HANDLE);

impl Drop for SafeHandle {
    fn drop(&mut self) {
        if self.0 != INVALID_HANDLE_VALUE {
            unsafe { CloseHandle(self.0); }
        }
    }
}

#[inline(never)]
pub fn get_final_path(path : &WStr) -> Result<Vec<u16>,WIN32_ERROR> {
    unsafe {
        let handle = SafeHandle(
            CreateFileW(
                path.as_ptr(),
                0,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                null_mut(),
                OPEN_EXISTING,
                FILE_FLAG_BACKUP_SEMANTICS,
                0));

        if handle.0 == INVALID_HANDLE_VALUE {
            return Err(GetLastError());
        }

        let mut buf : Vec<u16> =
            match GetFinalPathNameByHandleW(handle.0, null_mut(), 0, VOLUME_NAME_DOS) {
                0 => return Err(GetLastError()),
                size_needed_with_zero=> Vec::with_capacity(size_needed_with_zero as usize)
            };

        match GetFinalPathNameByHandleW(handle.0, buf.as_mut_ptr(), buf.capacity() as u32, VOLUME_NAME_DOS) {
            0 => return Err(GetLastError()),
            path_len => buf.set_len(path_len as usize)
        }

        Ok(buf)
    }
}


