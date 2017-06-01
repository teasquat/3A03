#![feature(lang_items)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate multiboot2;

mod vga_buffer;

use vga_buffer::Writer;
use core::fmt::Write;

#[lang = "eh_personality"]
extern fn eh_personality() {

}

#[no_mangle]
#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {
	loop {}
}


#[no_mangle]
pub extern fn kmain(multiboot_info_address: usize) -> ! {

	let mut writer = Writer {
		col: 0,
		row: 0,
	};

	write!(writer, "Hello world!\n").unwrap();


		let boot_info = unsafe{ multiboot2::load(multiboot_info_address) };
		let memory_map_tag = boot_info.memory_map_tag()
	    .expect("Memory map tag required");

		write!(writer, "memory areas:\n");
		for area in memory_map_tag.memory_areas() {
		    write!(writer,
				"    start: 0x{:x}, length: 0x{:x}\n",
		        area.base_addr, area.length);
		}

	loop {}
}
