#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use interface::init_interface;
use kernel::{
    hlt_loop, init,
    memory::paging::{self, BootInfoFrameAllocator},
};
use redos_std::println;
use x86_64::VirtAddr;

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { paging::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    init_interface();

    hlt_loop();
}
entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:?}", info);
    hlt_loop();
}
