#![feature(lang_items,const_fn)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate multiboot2;

#[macro_use]
mod vga_buffer;

use vga_buffer::WRITER;
use core::fmt::Write;

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[no_mangle]
#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {
    loop {}
}


#[no_mangle]
pub extern fn kmain(multiboot_info_address: usize) -> ! {

    println!("Hello world!");


    let boot_info = unsafe { multiboot2::load(multiboot_info_address) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}",
                 area.base_addr,
                 area.length);
    }

    loop {}
}
