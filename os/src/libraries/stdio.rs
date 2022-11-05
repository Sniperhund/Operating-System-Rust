use crate::kernel::{interrupts::keyboard::{newKey, lastKey}, stdout::_removeChar};
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

/// This function doesn't remove new line characters
pub fn readline() -> String {
    let mut lastChar: char = '\0';
    let mut result: String = String::new();
    while lastChar != '\n' {
        lastChar = getchar();
        if lastChar as u8 == 0x08 { // 0x08 is backspace
            result.pop();

            _removeChar();
        } else {
            result.push(lastChar);

            // Remove this when done with the shell
            print!("{}", lastChar);
        }
    }

    result
}