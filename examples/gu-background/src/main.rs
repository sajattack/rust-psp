#![no_std]
#![no_main]

use core::ffi::c_void;
use psp::sys::gu::PixelFormat;
use psp::sys::ge;

psp::module!("gu_background", 1, 1);

static mut list: psp::Align16<[u32; 262144]> = psp::Align16([0;262144]);

fn psp_main() {
    psp::enable_home_button();

    unsafe {
        let fbp0 = get_static_vram_buffer(512, 272, psp::sys::gu::PixelFormat::Psm8888);
        let fbp1 = get_static_vram_buffer(512, 272, psp::sys::gu::PixelFormat::Psm8888);
        let zbp = get_static_vram_buffer(512, 272, psp::sys::gu::PixelFormat::Psm4444);

        psp::sys::gu::sce_gu_init();
        psp::sys::gu::sce_gu_start(
            psp::sys::gu::Context::Direct as i32,
            &mut list as *mut _ as *mut c_void
        );
        psp::sys::gu::sce_gu_draw_buffer(psp::sys::gu::PixelFormat::Psm8888, fbp0, 512);
        psp::sys::gu::sce_gu_disp_buffer(480, 272, fbp1, 512);
        psp::sys::gu::sce_gu_depth_buffer(zbp, 512);
        psp::sys::gu::sce_gu_offset(2048 - (480/2), 2048 - (272/2));
        psp::sys::gu::sce_gu_viewport(2048, 2048, 480, 272);
        psp::sys::gu::sce_gu_depth_range(65535, 0);
        psp::sys::gu::sce_gu_scissor(0, 0, 480, 272);
        psp::sys::gu::sce_gu_enable(psp::sys::gu::State::ScissorTest);
        psp::sys::gu::sce_gu_finish();
        psp::sys::gu::sce_gu_sync(
            psp::sys::gu::SyncMode::SyncFinish,
            psp::sys::gu::SyncBehaviorWhat::SyncWhatDone
        );
        psp::sys::display::sce_display_wait_vblank_start();
        psp::sys::gu::sce_gu_display(true);

        loop {
            psp::sys::gu::sce_gu_start(
                psp::sys::gu::Context::Direct as i32,
                &mut list as *mut _ as *mut c_void
            );
            psp::sys::gu::sce_gu_clear_color(0xff554433);
            psp::sys::gu::sce_gu_clear_depth(0);
            psp::sys::gu::sce_gu_clear(
                psp::sys::gu::ClearBuffer::COLOR_BUFFER_BIT |
                psp::sys::gu::ClearBuffer::DEPTH_BUFFER_BIT
            );
            psp::sys::gu::sce_gu_finish();
            psp::sys::gu::sce_gu_sync(
                psp::sys::gu::SyncMode::SyncFinish,
                psp::sys::gu::SyncBehaviorWhat::SyncWhatDone
            );
            psp::sys::display::sce_display_wait_vblank_start();
            psp::sys::gu::sce_gu_swap_buffers();
        }
    }
}

fn get_memory_size(width: i32, height: i32, psm: PixelFormat) -> i32 {
    match psm {
        PixelFormat::PsmT4 => (width * height) >> 1,
        PixelFormat::PsmT8 => width * height,

        PixelFormat::Psm5650
        | PixelFormat::Psm5551
        | PixelFormat::Psm4444
        | PixelFormat::PsmT16 => {
            2 * width * height
        }

        PixelFormat::Psm8888 | PixelFormat::PsmT32 => 4 * width * height,

        _ => unimplemented!(),
    }
}

unsafe fn get_static_vram_buffer(width: i32, height: i32, psm: PixelFormat) -> *mut c_void {
    static mut STATIC_OFFSET: i32 = 0;

    let mem_size = get_memory_size(width, height, psm);
    let result = STATIC_OFFSET as *mut _;

    STATIC_OFFSET += mem_size;

    result
}

unsafe fn get_static_vram_texture(width: i32, height: i32, psm: PixelFormat) -> *mut c_void {
    let result = get_static_vram_buffer(width, height, psm);

    ((result as u32) + (ge::sce_ge_edram_get_addr() as u32)) as *mut _
}