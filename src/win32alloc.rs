use alloc::alloc::{GlobalAlloc, Layout};

use windows_sys::Win32::System::Memory::{GetProcessHeap, HEAP_NONE, HEAP_ZERO_MEMORY, HeapAlloc, HeapFree, HeapReAlloc};

pub struct Heapalloc;

unsafe impl GlobalAlloc for Heapalloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        HeapAlloc(
            GetProcessHeap(),
            HEAP_NONE,
            _layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        HeapFree(
            GetProcessHeap(),
            HEAP_NONE,
            _ptr as *mut core::ffi::c_void) ;

    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        HeapAlloc(
        GetProcessHeap(),
         HEAP_ZERO_MEMORY,
        layout.size()) as *mut u8
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        HeapReAlloc(
            GetProcessHeap(),
            HEAP_ZERO_MEMORY,
            ptr as *mut ::core::ffi::c_void,
            new_size)
        as *mut u8
    }
}