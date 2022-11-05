use x86_64::structures::idt::InterruptStackFrame;
use lazy_static::lazy_static;
use crate::{kernel, print, println};

pub static mut lastKey: char = '\0';
pub static mut newKey: bool = false;

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    unsafe { lastKey = character; newKey = true; }
                },
                DecodedKey::RawKey(key) => {},
            }
        }
    }

    unsafe {
        kernel::interrupts::PICS.lock().notify_end_of_interrupt(kernel::interrupts::InterruptIndex::Keyboard.as_u8());
    }
}