const HEIGHT: usize = 25;
const WIDTH: usize = 160;

pub struct Writer {
    pub col: usize,
    pub row: usize,
}

impl Writer {
    pub fn print(&mut self, string: &str) {
        for byte in string.bytes() {
            self.write_byte(byte);
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {

                if self.col > WIDTH {
                    self.new_line();
                }

                unsafe {
                    let vga = (0xb8000 + 2 * self.col + WIDTH * self.row) as *mut u16;

                    *vga = 0x02 << 8 | byte as u16;
                }

                self.col += 1;
            }
        }
    }

    fn new_line(&mut self) {
        self.col  = 0;
        self.row += 1;
    }
}
