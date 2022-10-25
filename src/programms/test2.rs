use vga::registers::PlaneMask;
use vga::vga::VGA;
use vga::writers::{Graphics320x240x256, GraphicsWriter};

pub fn change_vgamode() {
    let mode = Graphics320x240x256::new();
    mode.set_mode();
    mode.clear_screen(0);

    let my_str = include_str!("../../test2.txt");

    let frame_buffer = mode.get_frame_buffer();
    let mut vga = VGA.lock();

    for (offset, character) in my_str.chars().enumerate() {
        let color;
        if character == '1' {
            color = 255;
        } else {
            color = 0;
        }
        unsafe {
            let plane_mask = 0x1 << (offset % 320 & 3);
            let offset = offset / 4;
            vga.sequencer_registers
                .set_plane_mask(PlaneMask::from_bits(plane_mask).unwrap());
            frame_buffer.add(offset).write(color);
        }
    }
}
