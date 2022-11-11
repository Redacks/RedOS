use vga::writers::{Graphics320x200x256, GraphicsWriter};

use crate::screen::vga_pixel::draw_pixel;

pub fn change_vgamode() {
    let mode = Graphics320x200x256::new();
    mode.set_mode();
    mode.clear_screen(0);

    for x in 0..50 {
        for y in 0..100 {
            draw_pixel(x, y, 120);
        }
    }
}
