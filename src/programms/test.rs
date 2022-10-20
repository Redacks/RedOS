use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter};

pub fn change_vgamode() {
    let mode = Graphics640x480x16::new();
    mode.set_mode();
    mode.clear_screen(Color16::Black);
    mode.draw_line((80, 60), (80, 420), Color16::White);
    mode.draw_line((80, 60), (540, 60), Color16::White);
    mode.draw_line((80, 420), (540, 420), Color16::White);
    mode.draw_line((540, 420), (540, 60), Color16::White);
    mode.draw_line((80, 90), (540, 90), Color16::White);
    for (offset, character) in "Hello World! ASM SUCKS".chars().enumerate() {
        mode.draw_character(240 + offset * 8, 72, character, Color16::White)
    }
}
