use crate::kernel::interrupts::keyboard::{newKey, lastKey};
use alloc::string::String;

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

// Returns a new line character.
pub fn readline() -> String {
    let mut lastChar: char = '\0';
    let mut result: String = String::new();
    while lastChar != '\n' {
        lastChar = getchar();
        result.push(lastChar);
    }

    result
}