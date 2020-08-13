#![no_std]
#![no_main]

psp::module!("sample_module", 1, 1);
extern crate alloc;
use alloc::alloc::Layout;
use core::time::Duration;

fn psp_main() {
    psp::enable_home_button();

    // Enable the VFPU
    unsafe {
        use psp::sys::{self, ThreadAttributes};
        sys::sceKernelChangeCurrentThreadAttr(0, ThreadAttributes::VFPU);
    }

    let mut size = 16;
    let mut cpu_dur = Duration::default();
    let mut cpu32_dur = Duration::default();
    let mut dmac_dur = Duration::default(); 
    let mut vfpu_dur = Duration::default();
    //loop {
        let src = unsafe { alloc::alloc::alloc(Layout::from_size_align_unchecked(size, 16)) };
        let dst = unsafe { alloc::alloc::alloc(Layout::from_size_align_unchecked(size, 16)) };
        cpu_dur = psp::benchmark(|| {
            for _ in 0..1000 {
                unsafe { memcpy(dst, src as *const u8, size); }
            }
        }, 10);

        cpu32_dur = psp::benchmark(|| {
            for _ in 0..1000 {
                unsafe { memcpy32(dst, src as *const u8, size); }
            }
        }, 10);


        dmac_dur = psp::benchmark(|| {
            for _ in 0..1000 {
                unsafe { psp::sys::sceDmacMemcpy(dst, src as *const u8, size); }
            }
        }, 10);

        vfpu_dur = psp::benchmark(|| {
            for _ in 0..1000 {
                unsafe { psp::sys::sceVfpuMemcpy(dst, src as *const u8, size); }
            }
        }, 10);

        unsafe { alloc::alloc::dealloc(src, Layout::from_size_align_unchecked(size, 16)); }
        unsafe { alloc::alloc::dealloc(dst, Layout::from_size_align_unchecked(size, 16)); }
        //if dmac_dur < cpu32_dur {
            //break;
        //}
        //size += 16
    //}
    psp::dprintln!("size: {}", size);
    psp::dprintln!("cpu: {}", cpu_dur.as_nanos());
    psp::dprintln!("cpu32: {}", cpu32_dur.as_nanos());
    psp::dprintln!("dmac: {}", dmac_dur.as_nanos());
    psp::dprintln!("vfpu: {}", vfpu_dur.as_nanos());
}

unsafe fn memcpy(dst: *mut u8, src: *const u8, num: usize) -> *mut u8 {
    for i in 0..num {
        *dst.add(i) = *src.add(i);
    }

    dst
}

unsafe fn memcpy32(dst: *mut u8, src: *const u8, num: usize) -> *mut u8 {
    let mut size = num;
    let mut dst32 = dst as *mut u32;
    let mut src32 = src as *const u32;
    while size > 3 {
        *dst32 = *src32;
        dst32 = dst32.add(1);
        src32 = src32.add(1);
        size = size.saturating_sub(4);
    }
    let mut dst = dst32 as *mut u8;
    let mut src = src32 as *const u8;
    while size > 0 {
        *dst = *src;
        dst = dst.add(1);
        src = src.add(1);
        size = size.saturating_sub(1);
    }

    dst
}
