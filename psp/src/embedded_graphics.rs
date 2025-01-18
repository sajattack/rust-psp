//! Interop between the `psp` crate and the 2D `embedded-graphics` crate.

use crate::sys;
use crate::{BUF_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};
use core::convert::TryInto;
use embedded_graphics_core::{draw_target::*, geometry::Size, pixelcolor::*, prelude::*, Pixel};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rgba8888(u32);

impl Rgba8888 where Self: RgbColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        let r_shifted = ((r & Self::MAX_R) as u32) << 16;
        let g_shifted = ((g & Self::MAX_G) as u32) << 8;
        let b_shifted = ((b & Self::MAX_B) as u32) << 0;

        Self(r_shifted | g_shifted | b_shifted)
    }
}

impl RgbColor for Rgba8888 {

    #[inline(always)]
    fn r(&self) -> u8 {
        (self.0 >> 16 & 0xff) as u8
    }
    #[inline(always)]
    fn g(&self) -> u8 {
        (self.0 >> 8 & 0xff) as u8
    }
    #[inline(always)]
    fn b(&self) -> u8 {
        (self.0 >> 0 & 0xff) as u8
    }

    const MAX_R: u8 = ((1usize << 8) - 1) as u8;
    const MAX_G: u8 = ((1usize << 8) - 1) as u8;
    const MAX_B: u8 = ((1usize << 8) - 1) as u8;

    const BLACK: Self = Self::new(0, 0, 0);
    const RED: Self = Self::new(Self::MAX_R, 0, 0);
    const GREEN: Self = Self::new(0, Self::MAX_G, 0);
    const BLUE: Self = Self::new(0, 0, Self::MAX_B);
    const YELLOW: Self = Self::new(Self::MAX_R, Self::MAX_G, 0);
    const MAGENTA: Self = Self::new(Self::MAX_R, 0, Self::MAX_B);
    const CYAN: Self = Self::new(0, Self::MAX_G, Self::MAX_B);
    const WHITE: Self = Self::new(Self::MAX_R, Self::MAX_G, Self::MAX_B);
}

impl PixelColor for Rgba8888 {
    type Raw = raw::RawU32;
}


impl From<raw::RawU32> for Rgba8888 {
    fn from(data: raw::RawU32) -> Self {
        let data = data.into_inner();
        Self(data)
    }
}



pub struct Framebuffer {
    vram_base: *mut u16,
}

impl Framebuffer {
    pub fn new() -> Self {
        unsafe {
            sys::sceDisplaySetMode(sys::DisplayMode::Lcd, 480, 272);
            let vram_base = (0x4000_0000u32 | sys::sceGeEdramGetAddr() as u32) as *mut u16;
            sys::sceDisplaySetFrameBuf(
                vram_base as *const u8,
                BUF_WIDTH as usize,
                sys::DisplayPixelFormat::Psm8888,
                sys::DisplaySetBufSync::NextFrame,
            );
            Framebuffer { vram_base }
        }
    }
}

impl DrawTarget for Framebuffer {
    type Error = core::convert::Infallible;
    type Color = Rgba8888;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics_core::Pixel<Self::Color>>,
    {
        for p in pixels.into_iter() {
            self.draw_pixel(p)?;
        }

        Ok(())
    }
}

impl OriginDimensions for Framebuffer {
    fn size(&self) -> Size {
        Size::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }
}

impl Framebuffer {
    fn draw_pixel(&mut self, pixel: Pixel<Rgba8888>) -> Result<(), core::convert::Infallible> {
        let Pixel(coord, color) = pixel;

        if let Ok((x @ 0..=SCREEN_WIDTH, y @ 0..=SCREEN_HEIGHT)) = coord.try_into() {
            unsafe {
                let ptr = (self.vram_base as *mut u32)
                    .offset(x as isize)
                    .offset((y * BUF_WIDTH) as isize);

                *ptr = (color.r() as u32) | ((color.g() as u32) << 8) | ((color.b() as u32) << 16);
            }
        }

        Ok(())
    }
}
