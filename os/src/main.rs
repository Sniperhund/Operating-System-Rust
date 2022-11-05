#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(abi_x86_interrupt)]
#![feature(ptr_internals)]
#![feature(const_mut_refs)]

mod kernel;
mod libraries;

use bootloader::{BootInfo, entry_point};

extern crate alloc;

entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    kernel::init_kernel(boot_info);

    loop {
        println!("{}", libraries::stdio::readline());
    }
}