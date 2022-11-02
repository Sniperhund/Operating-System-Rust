use crate::kernel::interrupts::keyboard::{newKey, lastKey};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::kernel::stdout::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn getchar() -> char {
    unsafe {
        while newKey != true {}
        newKey = false;
        lastKey
    }
}