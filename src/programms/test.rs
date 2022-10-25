use vga::colors::Color16;
use vga::registers::{PlaneMask, WriteMode};
use vga::vga::VGA;
use vga::writers::{Graphics640x480x16, GraphicsWriter};

pub fn change_vgamode() {
    let mode = Graphics640x480x16::new();
    mode.set_mode();
    mode.clear_screen(Color16::Black);

    let my_str = include_str!("../../test.txt");
    set_write_mode_2();
    let frame_buffer = mode.get_frame_buffer();
    let mut vga = VGA.lock();

    for (offset, character) in my_str.chars().enumerate() {
        let color;
        if character == '1' {
            color = Color16::White;
        } else {
            color = Color16::Black;
        }

        vga.graphics_controller_registers
            .set_bit_mask(0x80 >> (offset % 640 & 0x07));
        let offset = offset / 8;
        unsafe {
            frame_buffer.add(offset).read_volatile();
            frame_buffer.add(offset).write_volatile(u8::from(color));
        }
    }
}

fn set_write_mode_2() {
    let mut vga = VGA.lock();
    vga.graphics_controller_registers
        .set_write_mode(WriteMode::Mode2);
    vga.graphics_controller_registers.set_bit_mask(0xFF);
    vga.sequencer_registers
        .set_plane_mask(PlaneMask::ALL_PLANES);
}
