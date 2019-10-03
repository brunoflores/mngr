#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mngr::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use mngr::{serial_print, serial_println};

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use mngr::allocator;
    use mngr::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    mngr::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mngr::test_panic_handler(info);
}

#[test_case]
fn simple_allocation() {
    serial_print!("simple_allocation...");
    let heap_value = Box::new(42);
    assert_eq!(*heap_value, 42);
    serial_println!("[ok]");
}
