use core::ffi::c_void;

type undefined4 = f32;

#[repr(align(4))]
struct Color {
    f1: undefined4,
    f2: undefined4,
    f3: undefined4,
    f4: undefined4,
}

pub unsafe extern "C" fn sceVfpuColorAdd(arg1: *mut Color, arg2: *mut Color, arg3: *mut Color) -> *mut Color {
    vfpu_asm! {
        lv.q C010, a1;
        lv.q C020, a2;
        vadd.q C000, C010, C020;
        sv.q C000, a0;
        :: "{4}"(arg1), "{5}"(arg2), "{6}"(arg3) : "memory" : "volatile"
    }

    arg1
}

pub unsafe extern "C" fn sceVfpuColorZero(color: *mut undefined4) -> *mut undefined4 {
    *color = 0;
    *color.offset(1) = 0;
    *color.offset(2) = 0;
    *color.offset(3) = 0;
    color
}

// TODO verify param color order
pub unsafe extern "C" fn sceVfpuColorSet(
    a: undefined4,
    b: undefined4,
    g: undefined4,
    r: undefined4, 
    color: *mut undefined4
) -> *mut undefined4 {
    *color = a;
    *color.offset(1) = b;
    *color.offset(2) = g;
    *color.offset(3) = r;
    color
}

// TODO verify param color order
pub unsafe extern "C" fn sceVfpuColorSetRGB(
    r: undefined4,
    g: undefined4,
    b: undefined4,
    color: *mut undefined4
) -> *mut undefined4 {
    *color = r;
    *color.offset(1) = g;
    *color.offset(2) = b;
    color
}
    

