#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]

use io::keyboard::PICS;
extern crate alloc;

pub mod config;
pub mod gdt;
pub mod interrupts;
pub mod io;
pub mod memory;
pub mod programms;
pub mod screen;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
