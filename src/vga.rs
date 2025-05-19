use core::cell::UnsafeCell;
use core::slice::from_raw_parts;
use core::marker::PhantomData;
use spin::{lazy::Lazy, RwLock};

pub const VGA_HEIGHT: usize = 25;
pub const VGA_WIDTH: usize = 80;

#[repr(u8)]
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
    pub const fn new(character: u8, background_color: VgaColor, foreground_color: VgaColor) -> Self {
        let color = ((background_color as u8) << 4) | (foreground_color as u8);

        Self {
            color,
            character,
        }       
    }
}

pub static VGA_BUFFER_LOCK: Lazy<RwLock<VgaBufferHandle>> = Lazy::new(|| RwLock::new(VgaBufferHandle(PhantomData)));

pub struct VgaBufferHandle(PhantomData<()>);

impl VgaBufferHandle {
    const VGA_BASE: usize = 0xb8000;

    pub fn get(&self, x: usize, y: usize) -> &VgaChar {
        let vga_slice = unsafe { core::slice::from_raw_parts(Self::VGA_BASE as *const VgaChar, VGA_WIDTH * VGA_HEIGHT) };

        &vga_slice[y * VGA_WIDTH + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut VgaChar {
        let vga_slice = unsafe {core::slice::from_raw_parts_mut(Self::VGA_BASE as *mut VgaChar, VGA_WIDTH * VGA_HEIGHT)};

        &mut vga_slice[y * VGA_WIDTH + x]
    }

    pub fn clear(&mut self) {
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                *self.get_mut(x, y) = VgaChar::default();
            }
        }
    }
}
