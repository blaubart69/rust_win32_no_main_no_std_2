use alloc::boxed::Box;
use alloc::vec::Vec;
use windows_sys::w;
use windows_sys::Win32::Foundation::WIN32_ERROR;
use crate::win32::{writeln_console,GetFinalPath};
use crate::wstr::WStr;

pub fn my_main(args : Vec<WStr>) -> Result<(),WIN32_ERROR> {
    //let first = argv as &[u16];


    for (idx, arg) in args.iter().enumerate() {
        writeln_console(arg)?;
    }

    let u = unsafe { WStr::from_pwstr(w!("berni")) };
    writeln_console(&u )?;

    if args.len() == 1 {
        let finalPath = GetFinalPath(&args[0])?;
        writeln_console(&WStr(&finalPath))?;
    }

    Ok(())
}

fn t1(v : &Vec<u16>, b : &Box<i32>) -> i32
{
    let v_sum : u16 = v.iter().sum();
    let s_vale : i32 = **b;
    v_sum as i32 + s_vale
}