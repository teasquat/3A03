use core::fmt;
use spin::Mutex;

//in char not bytes
//const HEIGHT: usize = 25;
const WIDTH: usize = 80;

pub struct Writer {
    pub col: usize,
    pub row: usize,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {

                if self.col > WIDTH {
                    self.new_line();
                }

                unsafe {
                    let vga = (0xb8000 + (self.col + WIDTH * self.row) * 2) as *mut u16;

                    *vga = 0x02 << 8 | byte as u16;
                }

                self.col += 1;
            }
        }
    }

    fn new_line(&mut self) {
        self.col = 0;
        self.row += 1;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
        Ok(())
    }
}

pub static WRITER: Mutex<Writer> = Mutex::new(Writer { col: 0, row: 0 });

macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = $crate::vga_buffer::WRITER.lock();
        writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
