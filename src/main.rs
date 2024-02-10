#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(asm_experimental_arch)]

extern crate alloc;

use alloc::format;
use alloc::alloc::{GlobalAlloc, Layout};
use core::arch::{asm};

fn get_stderr() -> *const u8 {
    // 0x488 == offset of stderr in _reent
    unsafe { __getreent().wrapping_add(0x488) }
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => (unsafe { fprintf(get_stderr(), "%s\0".as_ptr(), format!($($arg)*).as_ptr()) });
}

#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}

pub struct LibdragonAllocator;

#[global_allocator]
pub static ALLOCATOR: LibdragonAllocator = LibdragonAllocator {};

unsafe impl GlobalAlloc for LibdragonAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let _align = layout.align();

        malloc(size as u32)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr);
    }
}

#[cfg(not(test))]
#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

extern "C" {
    // keep rust optimizer from removing the entry point
    fn _start() -> !;

    fn debug_init_isviewer() -> ();

    fn __getreent() -> *const u8;
    fn fprintf(fd: *const u8, msg: *const u8, ...) -> u32;

    fn malloc(size: u32) -> *mut u8;
    fn free(ptr: *mut u8) -> ();
}

// libdragon's entry point calls C main
#[no_mangle]
extern "C" fn main() -> ! { rust_entrypoint(); }

#[no_mangle]
extern "C" fn rust_entrypoint() -> ! {
    rust_main();
    loop {}
}

fn rust_main() {
    unsafe {
        debug_init_isviewer();
    }
    
    eprintln!("Hello from rust! Here's a number: {}      ", 5u32);

    loop {
        unsafe { asm!("nop"); }
    }
}


#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    eprintln!("panic: {info}");
    
    loop {}
}
