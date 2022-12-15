#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]

extern crate alloc;

pub mod gdt;
pub mod interrupts;
pub mod memory;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
