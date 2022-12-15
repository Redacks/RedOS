use core::fmt::{self, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use vga::{
    colors::{Color16, TextModeColor},
    writers::{ScreenCharacter, Text80x25, TextWriter},
};
use x86_64::instructions::interrupts;

pub struct Console {
    vga_buffer: Text80x25,
    position: usize,
}

impl Console {
    pub fn new(vga_buffer: Text80x25) -> Self {
        vga_buffer.set_mode();
        vga_buffer.disable_cursor();
        Self {
            vga_buffer,
            position: 0,
        }
    }

    pub fn write_character(&mut self, c: char) {
        self.write_screen_character(ScreenCharacter::new(
            c as u8,
            TextModeColor::new(Color16::White, Color16::Black),
        ))
    }

    pub fn write_screen_character(&mut self, mut c: ScreenCharacter) {
        if c.get_character() == '\n' as u8 {
            self.position = ((self.position - (self.position % 80)) / 80 + 1) * 80;
            return;
        }
        match c.get_character() as u8 {
            0x20..=0x7e => (),
            _ => c = ScreenCharacter::new(0xfe as u8, c.get_color()),
        }
        self.vga_buffer.write_character(
            self.position % 80,
            (self.position - self.position % 80) / 80,
            c,
        );
        self.position += 1;
    }

    pub fn write_string(&mut self, s: &str) {
        for c in s.chars() {
            if self.position > 80 * 25 {
                return;
            }
            self.write_character(c);
        }
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
pub fn _print(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        CONSOLE.lock().write_fmt(args).unwrap();
    })
}
lazy_static! {
    pub static ref CONSOLE: Mutex<Console> = Mutex::new(Console::new(Text80x25::new()));
}
