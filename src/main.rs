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
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    //panic!("allocation error: {:?}", layout)
    loop {}
}

#[used]
#[no_mangle]
pub static _fltused: i32 = 0;

use core::panic::PanicInfo;

use alloc::vec::{Vec};
use alloc::boxed::{Box};
use windows_sys::core::PWSTR;

use windows_sys::Win32::System::Environment::GetCommandLineW;
use windows_sys::Win32::System::Memory::LocalFree;
use windows_sys::Win32::System::Threading::ExitProcess;
use windows_sys::Win32::UI::Shell::CommandLineToArgvW;

use crate::win32alloc::Heapalloc;

mod bumsti;

#[panic_handler]
#[no_mangle]
pub extern fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[no_mangle] // don't mangle the name of this function
pub extern fn mainCRTStartup() {

    unsafe {
        let mut argc : i32 = 0;
        let argv= CommandLineToArgvW(GetCommandLineW(), &mut argc);


        //let arg0 = alloc::string::String::from_utf16(*argv[0]).unwrap();
        //bumsti::write_stdout(arg0.as_str());

        let beeMainExitCode = beeMain(argc, argv);

        LocalFree(argv as isize);
        ExitProcess(beeMainExitCode);
    }
}

fn beeMain(argc : i32, argv : *mut PWSTR) -> u32 {
    //let first = argv as &[u16];

    let mut first_veccal : Vec<u16> = Vec::new();

    first_veccal.push(128);

    let boxxi = Box::new(5);
    let rc = t1(&first_veccal, &boxxi);
    rc as u32
}

fn t1(v : &Vec<u16>, b : &Box<i32>) -> i32
{
    let v_sum : u16 = v.iter().sum();
    let s_vale : i32 = **b;
    v_sum as i32 + s_vale
}