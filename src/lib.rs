#![no_std]
#![feature(abi_x86_interrupt)]

pub mod interrupts;
pub mod screen;

pub fn init() {
    interrupts::init_idt();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}