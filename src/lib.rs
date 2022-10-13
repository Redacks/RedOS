#![no_std]
#![feature(abi_x86_interrupt)]

use io::keyboard::PICS;

pub mod gdt;
pub mod interrupts;
pub mod io;
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
