//! Interop between the `psp` crate and the 2D `embedded-graphics` crate.

use crate::sys;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, BUF_WIDTH};
use core::ffi::c_void;
use core::convert::TryFrom;
use embedded_graphics::{
    drawable::Pixel,
    geometry::{Size, Dimensions},
    pixelcolor::Rgb888,
    pixelcolor::raw::{RawU24, RawData},
    geometry::Point,
    DrawTarget,
    image::{Image, ImageDimensions, IntoPixelIter},
};
use alloc::alloc::{alloc, Layout};

use crate::vram_alloc::{VramMemChunk, self};

pub struct PspDisplay {
    draw_buf: VramMemChunk, 
    disp_buf: VramMemChunk,
    texture_buf: VramMemChunk,
    pub size: Size,
}

static mut LIST: crate::Align16<[u32; 0x40000]> = crate::Align16([0; 0x40000]);


#[repr(C, align(4))]
struct Vertex {
    u: f32,
    v: f32,
    x: f32,
    y: f32,
    z: f32
}

static VERTICES: crate::Align16<[Vertex; 2]> = crate::Align16([
    Vertex { u: 0.0, v: 0.0, x: 0.0, y: 0.0, z: 0.0},
    Vertex { u: 480.0, v: 272.0, x: 480.0, y: 272.0, z: 0.0},
]);

impl PspDisplay {
    pub fn new() -> Self {
        unsafe {
            let size = Size::new(480, 272);

            sys::sceKernelChangeCurrentThreadAttr(0, sys::ThreadAttributes::VFPU);
            sys::sceDisplaySetMode(sys::DisplayMode::Lcd, SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize);

            let mut allocator = vram_alloc::get_vram_allocator().unwrap();
            let disp = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, sys::TexturePixelFormat::Psm8888);
            let draw = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, sys::TexturePixelFormat::Psm8888);
            let tex = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, sys::TexturePixelFormat::Psm8888);
            sys::sceGuInit();
            sys::sceGuStart(
                sys::GuContextType::Direct,
                &mut LIST as *mut _ as *mut c_void,
            );
            sys::sceGuDrawBuffer(sys::DisplayPixelFormat::Psm8888, draw.as_mut_ptr_from_zero() as *mut c_void, BUF_WIDTH as i32);
            sys::sceGuDispBuffer(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, disp.as_mut_ptr_from_zero() as *mut c_void, BUF_WIDTH as i32);
            sys::sceGuOffset(2048 - (SCREEN_WIDTH / 2), 2048 - (SCREEN_HEIGHT / 2));
            sys::sceGuViewport(2048, 2048, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
            sys::sceGuScissor(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
            sys::sceGuEnable(sys::GuState::ScissorTest);
            sys::sceGuEnable(sys::GuState::Texture2D);

            sys::sceGumMatrixMode(sys::MatrixMode::Projection);
            sys::sceGumLoadIdentity();
	    sys::sceGumOrtho(0.0, 480.0, 272.0, 0.0, -30.0, 30.0);

            sys::sceGumMatrixMode(sys::MatrixMode::View);
            sys::sceGumLoadIdentity();
            sys::sceGumMatrixMode(sys::MatrixMode::Model);
            sys::sceGumLoadIdentity();

            sys::sceGuTexMode(sys::TexturePixelFormat::Psm8888, 0, 0, 0);
            sys::sceGuTexFunc(sys::TextureEffect::Replace, sys::TextureColorComponent::Rgb);
            sys::sceGuTexFilter(sys::TextureFilter::Linear, sys::TextureFilter::Linear);
            sys::sceGuTexScale(1.0, 1.0);
            sys::sceGuTexOffset(0.0, 0.0);
            sys::sceGuTexWrap(sys::GuTexWrapMode::Clamp, sys::GuTexWrapMode::Clamp);

            sys::sceGuFinish();
            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
            sys::sceGuDisplay(true);

            Self { draw_buf: draw, disp_buf: disp, texture_buf: tex, size }
        }
    }

    #[inline]
    fn point_to_index(&self, point: Point) -> Option<usize> {
        if let Ok((x, y)) = <(u32, u32)>::try_from(point) {
            if x < BUF_WIDTH && y < self.size.height {
                return Some((x + y * BUF_WIDTH) as usize);
            }
        }
        None
    }

    pub fn flush(&mut self) {
        unsafe { 
            sys::sceGuStart(sys::GuContextType::Direct, &mut LIST.0 as *mut _ as *mut _);

            sys::sceGuTexImage(sys::MipmapLevel::None, 512, 512, 512, self.texture_buf.as_mut_ptr_direct_to_vram() as *const _ as *const _);

            sys::sceGumDrawArray(
                sys::GuPrimitive::Sprites, 
                sys::VertexType::TEXTURE_32BITF | sys::VertexType::TRANSFORM_2D | sys::VertexType::VERTEX_32BITF,
                2,
                core::ptr::null_mut(), 
                &VERTICES as *const _ as *const _
            );

            sys::sceGuFinish();
            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
        }
    }
}

impl Drop for PspDisplay {
    fn drop(&mut self) {
        unsafe {
            sys::sceGuTerm()
        }
    }
}

impl DrawTarget<Rgb888> for PspDisplay {
    type Error = core::convert::Infallible;

    fn draw_pixel(&mut self, pixel: Pixel<Rgb888>) -> Result<(), Self::Error> {
        let Pixel(point, color) = pixel;
        if let Some(index) = self.point_to_index(point) {
            unsafe {
                *(self.texture_buf.as_mut_ptr_direct_to_vram() as *mut u32).add(index) = 0xFF << 24 | rgb_to_bgr(RawU24::from(color).into_inner());
            }
        }
        Ok(())
    }

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Rgb888>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            if let Some(index) = self.point_to_index(point) {
                unsafe {
                    *(self.texture_buf.as_mut_ptr_direct_to_vram() as *mut u32).add(index) =0xFF << 24 |  rgb_to_bgr(RawU24::from(color).into_inner());
                }
            }
        }

        Ok(())
    }

        fn clear(&mut self, color: Rgb888) -> Result<(), Self::Error> {
        unsafe {
            sys::sceGuStart(sys::GuContextType::Direct, &mut LIST.0 as *mut _ as *mut _);
            sys::sceGuDrawBufferList(sys::DisplayPixelFormat::Psm8888, self.texture_buf.as_mut_ptr_from_zero() as *mut c_void, BUF_WIDTH as i32);
            sys::sceGuClearColor(rgb_to_bgr(RawU24::from(color).into_inner()));
            sys::sceGuClear(sys::ClearBuffer::COLOR_BUFFER_BIT | sys::ClearBuffer::FAST_CLEAR_BIT);

            sys::sceGuDrawBufferList(sys::DisplayPixelFormat::Psm8888, self.draw_buf.as_mut_ptr_from_zero() as *mut c_void, BUF_WIDTH as i32);
            sys::sceGuFinish();
            sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
        }
        Ok(())
    }

    fn draw_image<'a, 'b, I>(&mut self, item: &'a Image<'b, I, Rgb888>) -> Result<(), Self::Error>
    where
        &'b I: IntoPixelIter<Rgb888>,
        I: ImageDimensions,
    {
        let width = item.size().width as i32;
        let height = item.size().height as i32;

        let pixels: alloc::vec::Vec<u32> = item.into_iter().map(|p| 0xFF << 24 | rgb_to_bgr(RawU24::from(p.1).into_inner())).collect();

        unsafe { 
            for y in 0..height {
                memcpy(self.texture_buf.as_mut_ptr_direct_to_vram().add(y as usize*BUF_WIDTH as usize*4).add(self.point_to_index(item.top_left()).unwrap()*4), pixels.as_ptr().add(y as usize*width as usize) as *const u8, width as isize*4);
            }
        }

        Ok(())
    }

    fn size(&self) -> Size {
        Size::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }
}

#[inline]
fn rgb_to_bgr(rgb: u32) -> u32 {
    core::intrinsics::bswap(rgb << 8 | rgb >> 24)
}

extern "C" {
    fn memcpy(dst: *mut u8, src: *const u8, num: isize) -> *mut u8;
}
