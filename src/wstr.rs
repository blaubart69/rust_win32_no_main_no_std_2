use core::ptr::slice_from_raw_parts;

pub struct WStr<'a> (pub &'a [u16] );

impl<'a> WStr<'a> {
    pub unsafe fn from_pwstr(str : *const u16) -> WStr<'a> {
            let mut len = 0;
            let ptr = str.offset(0);
            loop {
                if *ptr.offset(len) == 0 {
                    break;
                }
                len += 1;
            }
            let x = slice_from_raw_parts(ptr, len as usize);
            WStr( &*x )
    }
    pub fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}