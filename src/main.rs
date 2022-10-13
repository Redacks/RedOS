#![no_std]
#![no_main]

use redos::{
    println,
    screen::{text_styling::Color, vga_display::set_color},
};

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use redos::hlt_loop;

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    redos::init();

    println!("Hello World");

    hlt_loop();
}
entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}
