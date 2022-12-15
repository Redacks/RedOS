#![no_std]
#![no_main]

pub mod console;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        ($crate::console::_print(format_args!($($arg)*)))
    };
}

#[macro_export]
macro_rules! println {
    () => (super::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n",format_args!($($arg)*)))
}