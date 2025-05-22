#![no_std]
#![no_main]
// #![feature(panic_info_message)]

mod lang_item;
mod sbi;
#[macro_use]
mod console;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
fn rust_main() {
    clear_bbs();
    println!("Hello, world!");
    panic!("Shotdown machine!");
}

fn clear_bbs() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
