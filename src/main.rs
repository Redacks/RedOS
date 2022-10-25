#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use redos::{
    memory::{
        allocator,
        paging::{self, BootInfoFrameAllocator},
    },
    println,
    screen::{text_styling::Color, vga_display::set_color},
};

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use redos::hlt_loop;
use x86_64::VirtAddr;

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    redos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { paging::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    set_color(Color::Red, Color::Black);
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    set_color(Color::Green, Color::Black);
    println!("Starting Test Programm!");
    redos::programms::test2::change_vgamode();

    hlt_loop();
}
entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
