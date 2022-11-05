pub mod stdout;
pub mod interrupts;
pub mod gdt;
pub mod memory;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use x86_64::{structures::paging::Page, VirtAddr};
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc, string::String};
use crate::{kernel, println};

pub fn init_kernel(boot_info: &'static BootInfo) {
    kernel::gdt::init();
    kernel::interrupts::init_idt();
    unsafe { kernel::interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { kernel::memory::paging::init(phys_mem_offset) };
    let mut frame_allocator = kernel::memory::paging::EmptyFrameAllocator;

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    kernel::memory::paging::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();

    let mut frame_allocator = unsafe {
        kernel::memory::paging::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    kernel::memory::allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("Panic: {}", _info);
    loop {}
}