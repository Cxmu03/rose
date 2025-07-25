use core::fmt;
use spin::{lazy::Lazy, RwLock};

macro_rules! print {
    ($($arg: tt)*) => {
        crate::vga::_print(format_args!($($arg)*));
    }
}

macro_rules! println {
    () => {
        print!("\n");
    };
    ($($arg: tt)*) => {
        crate::vga::print!("{}\n", format_args!($($arg)*));
    }
}

pub(crate) use print;
pub(crate) use println;

pub const VGA_HEIGHT: usize = 26;
pub const VGA_WIDTH: usize = 80;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum VgaColor {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    Gray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    LightMagenta = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

#[repr(C, align(2))]
#[derive(Copy, Clone, Default)]
pub struct VgaChar {
    character: u8,
    color: u8,
}

impl VgaChar {
    pub const fn new(
        character: char,
        background_color: VgaColor,
        foreground_color: VgaColor,
    ) -> Self {
        let color = ((background_color as u8) << 4) | (foreground_color as u8);
        let character = character as u8;

        Self { color, character }
    }

    pub const fn empty() -> Self {
        Self::new(' ', VgaColor::Black, VgaColor::White)
    }

    pub const fn empty_with_color(background_color: VgaColor, foreground_color: VgaColor) -> Self {
        Self::new(' ', background_color, foreground_color)
    }
}

pub static VGA_BUFFER_LOCK: Lazy<RwLock<VgaWriter>> = Lazy::new(|| {
    RwLock::new(VgaWriter {
        col_position: 0,
        foreground_color: VgaColor::White,
        background_color: VgaColor::Black,
    })
});

pub struct VgaWriter {
    col_position: usize,
    pub foreground_color: VgaColor,
    pub background_color: VgaColor,
}

impl VgaWriter {
    const VGA_BASE: usize = 0xb8000;

    fn get_slice(&self) -> &'static [VgaChar] {
        unsafe {
            core::slice::from_raw_parts(Self::VGA_BASE as *const VgaChar, VGA_WIDTH * VGA_HEIGHT)
        }
    }

    fn get_slice_mut(&mut self) -> &'static mut [VgaChar] {
        unsafe {
            core::slice::from_raw_parts_mut(Self::VGA_BASE as *mut VgaChar, VGA_WIDTH * VGA_HEIGHT)
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &VgaChar {
        let vga_slice = self.get_slice();

        &vga_slice[y * VGA_WIDTH + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut VgaChar {
        let vga_slice = self.get_slice_mut();

        &mut vga_slice[y * VGA_WIDTH + x]
    }

    fn new_line(&mut self) {
        self.col_position = 0;
        let vga_slice = self.get_slice_mut();

        for y in 1..VGA_HEIGHT {
            let previous_row_start = (y - 1) * VGA_WIDTH;
            let row_start = y * VGA_WIDTH;
            let row_end = row_start + VGA_WIDTH;
            vga_slice.copy_within(row_start..row_end, previous_row_start);
        }

        let last_row_start = (VGA_HEIGHT - 1) * VGA_WIDTH;
        let last_row_end = last_row_start + VGA_WIDTH;
        vga_slice[last_row_start..last_row_end].fill(VgaChar::empty());
    }

    pub fn write_char(&mut self, c: char) {
        if c == '\n' {
            self.new_line();
            return;
        }

        let vga_char = VgaChar::new(c, self.background_color, self.foreground_color);
        *self.get_mut(self.col_position, VGA_HEIGHT - 1) = vga_char;
        self.col_position += 1;

        if self.col_position >= VGA_WIDTH {
            self.new_line();
        }
    }

    pub fn clear(&mut self) {
        self.get_slice_mut().fill(VgaChar::empty_with_color(self.background_color, self.foreground_color));

        self.col_position = 0;
    }
}

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.chars().for_each(|c| self.write_char(c));

        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    VGA_BUFFER_LOCK.write().write_fmt(args).unwrap();
}
