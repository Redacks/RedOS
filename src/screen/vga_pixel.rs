use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use x86_64::instructions::interrupts;

const SCREEN_WIDTH: usize = 320;
const SCREEN_HEIGHT: usize = 200;

struct Buffer {
    pixels: [[Volatile<u8>; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

pub struct Drawer {
    buffer: &'static mut Buffer,
}
impl Drawer {
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u8) {
        if x > SCREEN_WIDTH {
            return;
        }
        if y > SCREEN_HEIGHT {
            return;
        }
        self.buffer.pixels[y][x].write(color);
    }
}

pub fn draw_pixel(x: usize, y: usize, color: u8) {
    interrupts::without_interrupts(|| DRAWER.lock().draw_pixel(x, y, color))
}

lazy_static! {
    pub static ref DRAWER: Mutex<Drawer> = Mutex::new(Drawer {
        buffer: unsafe { &mut *(0xA0000 as *mut Buffer) }
    });
}
