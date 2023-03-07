#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![no_std]
#![no_main]
#![windows_subsystem = "console"]   // set Entrypoint to "mainCRTStartup"

extern crate alloc;

pub mod win32alloc;
#[global_allocator]
static BEE_NO_STD_ALLOCATOR: Heapalloc = win32alloc::Heapalloc;

#[alloc_error_handler]
fn alloc_error_handler(_layout: alloc::alloc::Layout) -> ! {
    //panic!("allocation error: {:?}", layout)
    loop {}
}

#[panic_handler]
#[no_mangle]
pub extern fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[used]
#[no_mangle]
pub static _fltused: i32 = 0;

use core::panic::PanicInfo;

use windows_sys::Win32::System::Environment::GetCommandLineW;
use windows_sys::Win32::System::Memory::LocalFree;
use windows_sys::Win32::System::Threading::ExitProcess;
use windows_sys::Win32::UI::Shell::CommandLineToArgvW;

use crate::win32alloc::Heapalloc;
use crate::wstr::WStr;

mod win32;
mod beemain;
mod wstr;

#[no_mangle] // don't mangle the name of this function
pub extern fn mainCRTStartup() {

    let args = unsafe {
        let mut argc: i32 = 0;
        let argv = CommandLineToArgvW(GetCommandLineW(), &mut argc);

        let args = (0..argc)
            .map( |idx| {
                let arg_ptr = *argv.offset(idx as isize);
                WStr::from_pwstr(arg_ptr)
            }).collect();

        args
        //let arg0 = alloc::string::String::from_utf16(*argv[0]).unwrap();
        //bumsti::write_stdout(arg0.as_str());
    };

    let bee_main_exit_code = beemain::my_main(args);

    unsafe {
        //LocalFree(argv as isize);

        let rc = match bee_main_exit_code {
            Ok(()) => 0,
            Err(winerr) => winerr
        };

        ExitProcess(rc);
    }
}

