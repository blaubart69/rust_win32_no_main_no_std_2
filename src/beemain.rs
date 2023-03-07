use alloc::boxed::Box;
use alloc::vec::Vec;
use windows_sys::Win32::Foundation::WIN32_ERROR;
use crate::win32::{writeln_console};
use crate::winstr::WinStr;

pub fn my_main(args : Vec<WinStr>) -> Result<(),WIN32_ERROR> {
    //let first = argv as &[u16];

    for (idx, arg) in args.iter().enumerate() {
        writeln_console(arg)?;
    }

    let mut first_veccal : Vec<u16> = Vec::new();

    first_veccal.push(128);

    let boxxi = Box::new(5);
    let rc = t1(&first_veccal, &boxxi);
    Ok(())
}

fn t1(v : &Vec<u16>, b : &Box<i32>) -> i32
{
    let v_sum : u16 = v.iter().sum();
    let s_vale : i32 = **b;
    v_sum as i32 + s_vale
}