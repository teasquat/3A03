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
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop {}
}


#[no_mangle]
pub extern fn kmain(multiboot_info_address: usize) -> ! {

    println!("Hello world!\n");

    let boot_info = unsafe { multiboot2::load(multiboot_info_address) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}",
                 area.base_addr,
                 area.length);
    }

    let elf_sections_tag = boot_info
        .elf_sections_tag()
        .expect("Elf-sections tag required");

    println!("kernel sections:");
    for section in elf_sections_tag.sections() {
        println!("    addr:  0x{:x}, size: 0x{:x}, flags: 0x{:x}",
                 section.addr,
                 section.size,
                 section.flags);
    }

    println!("");

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();
    let kernel_end = elf_sections_tag
        .sections()
        .map(|s| s.addr + s.size)
        .max()
        .unwrap();


    println!("kernel:");
    println!("    start: 0x{:x}", kernel_start);
    println!("    end:   0x{:x}", kernel_end);

    let multiboot_start = multiboot_info_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("multiboot:");
    println!("    start: 0x{:x}", multiboot_start);
    println!("    end:   0x{:x}", multiboot_end);

    loop {}
}
