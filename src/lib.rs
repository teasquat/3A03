#![feature(lang_items)]
#![no_std]

mod vga_buffer;
use vga_buffer::Writer;

#[lang = "eh_personality"]
extern fn eh_personality() {

}

#[lang = "panic_fmt"]
extern fn rust_begin_panic() -> ! {
	loop {}
}


#[no_mangle]
pub extern fn kmain() -> ! {

	let mut writer = Writer {
		col: 0,
		row: 0,
	};

	writer.print("Hello, World!\nYou're welcome here.");
	writer.print(" Hello Hello");

	loop {}
}
