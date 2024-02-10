#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(asm_experimental_arch)]

extern crate alloc;

use core::arch::{asm};
use linked_list_allocator::LockedHeap;

pub mod isviewer;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::isviewer::write_fmt(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[cfg(not(test))]
#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

extern "C" {
    // keep rust optimizer from removing the entry point
    fn _start() -> !;

    fn debug_init_isviewer() -> ();
    fn fprintf(fd: u32, msg: *const u8) -> ();
}

// libdragon's entry point calls C main
#[no_mangle]
extern "C" fn main() -> !{
    rust_entrypoint();
}

#[no_mangle]
extern "C" fn rust_entrypoint() -> ! {
    // this will conflict with libdragon
    extern "C" {
        static __bss_end: u32;
    }
    
    let heap_start = (unsafe { &__bss_end as *const u32 as u32 } & 0x1FFF_FFFF) | 0xA000_0000; // uncached and unmapped
    let heap_size = (4 * 1024 * 1024) - unsafe { __bss_end }; // Remaining unused RDRAM (increase 4 -> 8, if using Expansion Pak)
    
    unsafe {
        ALLOCATOR.lock().init(heap_start as *mut u8, heap_size as usize);
    }
    
    rust_main();
    loop {}
}

fn rust_main() {
    unsafe {
        debug_init_isviewer();
        fprintf(2, "Hello, world!\n".as_ptr());
    }
    
    loop {
        unsafe { asm!("nop"); }
    }
}


#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    println!("panic: {info}");
    
    loop {}
}
