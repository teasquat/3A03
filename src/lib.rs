#![feature(lang_items)]
#![no_std]

extern crate rlibc;

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
pub extern fn kmain() -> ! {

	let mut writer = Writer {
		col: 0,
		row: 0,
	};

	write!(writer, "Hello {}\n{}", "world!", 20 * 7);

	loop {}
}
