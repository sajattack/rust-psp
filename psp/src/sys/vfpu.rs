use core::ffi::c_void;
use super::types::{ScePspFVector2, ScePspIVector2, ScePspVector2};

type undefined4 = f32;

#[repr(align(4))]
pub struct Color {
    f1: undefined4,
    f2: undefined4,
    f3: undefined4,
    f4: undefined4,
}

//TODO: there's a static table of floats at 0xd7a0c, should we duplicate it or just
// substitute values where they're used?

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2SignFloat(
    mut param_1: *mut ScePspFVector2,
    mut param_2: *mut ScePspFVector2,
)  -> *mut ScePspFVector2 {
    let mut fVar1: f32 = -1.0;
    if (*param_2).x < 0.0
    || {
        fVar1 = 1.0;
        0.0 < (*param_2).x
    } {
        (*param_1).x = fVar1;
    } else { (*param_1).x = 0.0 }
        fVar1 = -1.0;
    if (*param_2).y < 0.0 
    || {
        fVar1 = 1.0;
        0.0 < (*param_2).y
    } {
        (*param_1).y = fVar1
    } else {
        (*param_1).y = 0.0;
    }
    param_1
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2SignInt(
    mut param_1: *mut ScePspIVector2,
    mut param_2: *mut ScePspIVector2)
 -> *mut ScePspIVector2 {
    let mut iVar1: i32 = -1;
    if 0 <= (*param_2).x {
        if (*param_2).x <= 0 {
            (*param_1).x = 0;
            iVar1 = -1;
            if 0 <= (*param_2).y {
                if (*param_2).y <= 0 {
                    (*param_1).y = 0;
                    return param_1
                }
                iVar1 = 1;
            }
            (*param_1).y = iVar1
        }
        iVar1 = 1
    }
    (*param_1).x = iVar1;
    param_1
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Set(
    x: f32,
    y: f32,
    vector: *mut ScePspFVector2
) -> *mut ScePspFVector2 {
    (*vector).x = x;
    (*vector).y = y;
    vector 
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Copy(
    dst: *mut ScePspFVector2,
    src: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    let temp = (*src).x;
    (*dst).y = (*src).y;
    (*dst).x = temp;
    dst 
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2PositiveZero(
    vector: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    (*vector).x = 0.0;
    (*vector).y = 0.0;
    vector 
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2NegativeZero(
    vector: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    (*vector).y = 0x8000_0000u32 as f32;
    (*vector).x = 0x8000_0000u32 as f32;
    vector 
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Ceil(
    arg1: *mut ScePspFVector2,
    arg2: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    //vfpu_asm! {
        //lv_s S000, 0(a1);
        //lv_s S001, 4(a1);
        //vf2id_p C000, C000, 0;
        //sv_s S000, 0(a0);
        //sv_s S001, 4(a0);
        //: : "{4}"(arg1), "{5}"(arg2) : "memory" : "volatile"
    //}
    arg1
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Trunc(
    arg1: *mut ScePspFVector2,
    arg2: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    //vfpu_asm! {
        //lv_s S000, 0(a1);
        //lv_s S001, 4(a1);
        //vf2iz_p C000, C000, 0;
        //sv_s S000, 0(a0);
        //sv_s S001, 4(a0);
        //: : "{4}"(arg1), "{5}"(arg2) : "memory" : "volatile"
    //}
    arg1
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Round(
    arg1: *mut ScePspFVector2,
    arg2: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    //vfpu_asm! {
        //lv_s S000, 0(a1);
        //lv_s S001, 4(a1);
        //vf2in_p C000, C000, 0;
        //sv_s S000, 0(a0);
        //sv_s S001, 4(a0);
        //: : "{4}"(arg1), "{5}"(arg2) : "memory" : "volatile"
    //}
    arg1
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Floor(
    arg1: *mut ScePspFVector2,
    arg2: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    //vfpu_asm! {
        //lv_s S000, 0(a1);
        //lv_s S001, 4(a1);
        //vf2iu_p C000, C000, 0;
        //sv_s S000, 0(a0);
        //sv_s S001, 4(a0);
        //: : "{4}"(arg1), "{5}"(arg2) : "memory" : "volatile"
    //}
    arg1
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2FromIVector(
    dst: *mut ScePspFVector2,
    src: *mut ScePspIVector2,
) -> *mut ScePspFVector2 {
    (*dst).y = (*src).y as f32;
    (*dst).x = (*src).x as f32;
    dst
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Add(
    arg1: *mut ScePspFVector2,
    arg2: *mut ScePspFVector2,
    arg3: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    (*arg1).y = (*arg2).y + (*arg3).y;
    (*arg1).x = (*arg2).x + (*arg3).x;
    arg1
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Sub(
    arg1: *mut ScePspFVector2,
    arg2: *mut ScePspFVector2,
    arg3: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    (*arg1).y = (*arg2).y - (*arg3).y;
    (*arg1).x = (*arg2).x - (*arg3).x;
    arg1
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Mul(
    arg1: *mut ScePspFVector2,
    arg2: *mut ScePspFVector2,
    arg3: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    (*arg1).y = (*arg2).y * (*arg3).y;
    (*arg1).x = (*arg2).x * (*arg3).x;
    arg1
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Div(
    arg1: *mut ScePspFVector2,
    arg2: *mut ScePspFVector2,
    arg3: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    (*arg1).y = (*arg2).y / (*arg3).y;
    (*arg1).x = (*arg2).x / (*arg3).x;
    arg1
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Neg(
    arg1: *mut ScePspFVector2,
    arg2: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    (*arg1).y = 0.0 - (*arg2).y;
    (*arg1).x = 0.0 - (*arg2).x;
    arg1
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Abs(
    arg1: *mut ScePspFVector2,
    arg2: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    (*arg1).y = core::intrinsics::fabsf32((*arg2).y);
    (*arg1).x = core::intrinsics::fabsf32((*arg2).x);
    arg1
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Lerp(
    arg1: f32,
    arg2: *mut ScePspFVector2,
    arg3: *mut ScePspFVector2,
    arg4: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    (*arg2).y = (*arg3).y + arg1 * ((*arg4).y - (*arg3).y);
    (*arg2).x = (*arg3).x + arg1 * ((*arg4).x - (*arg3).x);
    arg2
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Scale(
    arg1: f32,
    arg2: *mut ScePspFVector2,
    arg3: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    (*arg2).y = (*arg3).y * arg1;
    (*arg2).x = (*arg3).x * arg1;
    arg2
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Hermite(
    arg1: f32,
    arg2: *mut ScePspFVector2,
    arg3: *mut ScePspFVector2,
    arg4: *mut ScePspFVector2,
    arg5: *mut ScePspFVector2,
    arg6: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    //vfpu_asm! {
        //.mips "addiu sp, sp, -0x10";
        //.mips "swc1 f12, 0x0(sp)";
        //lv_s S000, 0(a1);
        //lv_s S001, 4(a1);
	//lv_s S010, 0(a2);
	//lv_s S011, 4(a2);
	//lv_s S020, 0(t0);
	//lv_s S021, 4(t0);
	//lv_s S030, 0(a3);
	//lv_s S031, 4(a3);
	//lv_s S202,sp;
	//vone_s S203;
	//vmul_s S201,S202,S202;
	//vpfxs [2],[1],[1],[-2];
	//vmov_q C100,C100;
	//vpfxs [-3],[-2],[-1],[3];
	//vmov_q C110,C110;
	//vmul_s S200,S201,S202;
	//vpfxs [0],[1],[0],[0];
	//vmov_q C120,C120;
	//vpfxs [1],[0],[0],[0];
	//vmov_q C130,C130;
	//vtfm4_q C210,E100,C200;
	//vtfm4_q C220,E000,C210;
	//sv_s S220,0(a0);
	//sv_s S221,4(a0);
        //.mips "addiu sp, sp, 0x10";

        //: : "{4}"(arg2), "{5}"(arg3), "{6}"(arg4), "{7}"(arg5), "{8}"(arg6), "{f12}"(arg1) : "memory" : "volatile"
    //}
    arg2
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Clamp(
    arg1: f32,
    arg2: f32,
    arg3: *mut ScePspFVector2,
    arg4: *mut ScePspFVector2
) -> *mut ScePspFVector2 {
    //vfpu_asm! {
	//.mips "mfc1 t0,f12";
	//.mips "mfc1 t1,f13";
	//.mips "mtv t0,S010";
	//.mips "mtv t1,S011";
	//lv_s S000, 0(a1);
	////lv_s S001,4(a1);
	//vpfxt [X,X,Z,W];
	//vmax_p C000,C000,C010;
	//vpfxt [Y,Y,Z,W];
	//vmin_p C000,C000,C010;
	//sv_s S000, 0(a0);
	////sv_s S001,4(a0);

	//: : "{4}"(arg3), "{5}"(arg4), "{f12}"(arg1), "{f13}"(arg2) : "memory" : "volatile"
    //}
    arg3
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Max(
    max: *mut ScePspFVector2,
    a: *mut ScePspFVector2,
    b: *mut ScePspFVector2
) {
    let mut fVar1: f32;
    let mut fVar2: f32;
    let fVar3: f32;
    fVar1 = (*b).x;
    if fVar1 < (*a).x {
        fVar1 = (*a).x;
    }
    fVar3 = (*a).y;
    fVar2 = (*b).y;
    (*max).x = fVar1;
    if fVar2 < fVar3 {
        fVar2 = fVar3;
    }
    (*max).y = fVar2;
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Min(
    min: *mut ScePspFVector2,
    a: *mut ScePspFVector2,
    b: *mut ScePspFVector2
) {
    let mut fVar1: f32;
    let mut fVar2: f32;
    let fVar3: f32;
    fVar1 = (*b).x;
    if (*a).x < fVar1 {
        fVar1 = (*a).x;
    }
    fVar3 = (*a).y;
    fVar2 = (*b).y;
    (*min).x = fVar1;
    if fVar3 < fVar2 {
        fVar2 = fVar3;
    }
    (*min).y = fVar2;
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2InnerProduct(
    arg1: *mut ScePspFVector2, 
    arg2: *mut ScePspFVector2
) -> f32 {
    (*arg1).x * (*arg2).x + (*arg1).y * (*arg2).y
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Funnel(
    arg1: *mut ScePspFVector2
) -> f32 {
    (*arg1).x + (*arg1).y 
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Average(
    arg1: *mut ScePspFVector2
) -> f32 {
    ((*arg1).x + (*arg1).y) * 0.5
}

//TODO: fix
//#[no_mangle]
//pub unsafe extern "C" fn sceVfpuVector2IsEqual(
    //arg1: *mut ScePspFVector2,
    //arg2: *mut ScePspFVector2,
//) -> i32 {
    //vfpu_asm! {
        //lv_s S000, 0(a0);
        //lv_s S001, 4(a0);
        //lv_s S010, 0(a1);
        //lv_s S011, 4(a1);
        //.mips "li v0, 0";
        //vcmp_p EQ, C000, C010
        //// bvtl ret (CC[5])
        //.mips "li v0, 1";
    //}
    //return 0// ret
//}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2IsZero(
    arg1: *mut ScePspFVector2
) -> bool {
    (*arg1).x as u32 | (*arg1).y as u32 & 0x7fff_ffff == 0
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Normalize(
    dst: *mut ScePspFVector2,
    src: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    //vfpu_asm! {
        //lv_s S000, 0(a1);
        //lv_s S001, 4(a1);
        //vdot_p S010,C000,C000;
        //vzero_s S011;
        //vcmp_s EZ,S010,S010;
        //vrsq_s S010,S010;
        //vcmovt_s S010,S011,CC[0];
        //vpfxd [-1:1,-1:1,M,M];
        //vscl_p C000,C000,S010;
        //sv_s S000, 0(a0);
        //sv_s S001, 4(a0);
	//: : "{4}"(dst), "{5}"(src) : "memory" : "volatile"
    //};
    dst
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Length(
    arg: *mut ScePspFVector2,
) /*-> f32*/ {
    //vfpu_asm! {
        //.mips "addiu $$sp, $$sp, -0x10";
        //lv_s S000, 0(a1);
        //lv_s S001, 4(a1);
        //vdot_p S000,C000,C000;
        //vsqrt_s S000,S000;
        //sv_s S000, 0(sp);
        //lwc1 f0, 0(sp);
        //.mips "jr ra";
        //.mips "addiu $$sp, $$sp, 0x10";
        //: : "{4}"(arg) : "memory" : "volatile"
    //};
}

pub unsafe extern "C" fn sceVfpuVector2Distance (
    arg1: *mut ScePspFVector2,
    arg2: *mut ScePspFVector2,
) /*-> f32*/ {
    //vfpu_asm! {
        //.mips "addiu sp,sp,-0x10";
        //lv_s S000, 0(a0);
        //lv_s S001, 4(a0);
        //lv_s S010, 0(a1);
        //lv_s S011, 4(a1);
        //vsub_p C000, C000, C010;
        //vdot_p S000, C000, C000;
        //vsqrt_s S000, S000;
        //sv_s S000, 0(sp);
        //lwc1 f0, 0(sp);
        //.mips "jr ra";
        //.mips "addiu sp, sp, 0x10";
	//: : "{4}"(arg1), "{5}"(arg2) : "memory" : "volatile"
    //}
}



#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorAdd(arg1: *mut Color, arg2: *mut Color, arg3: *mut Color) -> *mut Color {
    vfpu_asm! {
        lv_q C010, 0(a1);
        lv_q C020, 0(a2);
        vadd_q C000, C010, C020;
        sv_q C000, 0(a0);
        : : "{4}"(arg1), "{5}"(arg2), "{6}"(arg3) : "memory" : "volatile"
    }
    arg1
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorZero(color: *mut undefined4) -> *mut undefined4 {
    *color = 0.0;
    *color.offset(1) = 0.0;
    *color.offset(2) = 0.0;
    *color.offset(3) = 0.0;
    color
}

// TODO verify param color order
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorSet(
    r: undefined4, 
    g: undefined4,
    b: undefined4,
    a: undefined4,
    color: *mut undefined4
) -> *mut undefined4 {
    *color = a;
    *color.offset(1) = b;
    *color.offset(2) = g;
    *color.offset(3) = r;
    color
}

// TODO verify param color order
#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorSetRGB(
    r: undefined4,
    g: undefined4,
    b: undefined4,
    color: *mut undefined4
) -> *mut undefined4 {
    *color = b;
    *color.offset(1) = g;
    *color.offset(2) = r;
    color
}

pub unsafe extern "C" fn sceVfpuColorCopy(
    dst: *mut undefined4,
    src: *mut undefined4,
) -> *mut undefined4 {
    *dst = *src;
    *dst.offset(1) = *src.offset(1);
    *dst.offset(2) = *src.offset(2);
    *dst.offset(3) = *src.offset(3);
    dst
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuMemcpy(
    mut dst8: *mut u8,
    mut src8: *const u8,
    mut size: usize,
) -> *mut u8 {
    if size == 0 {
        return dst8
    }
    

    let mut dst32 = dst8 as *mut u32;
    let mut src32 = src8 as *const u32;

    if ((src8 as u32)&0xF) == 0 //Both src and dst are 16byte aligned
    {
        while size > 63 {
            vfpu_asm! {
                lv_q C000, 0(a1);
                lv_q C010, 16(a1);
                lv_q C020, 32(a1);
                lv_q C030, 48(a1);
                sv_q C000, 0(a0);
                sv_q C010, 16(a0);
                sv_q C020, 32(a0);
                sv_q C030, 48(a0);
                //.mips "addiu $$a2, $$a2, -64";
                .mips "addiu $$a1, $$a1, 64";
                .mips "addiu $$a0, $$a0, 64";
                : : "{4}"(dst8), "{5}"(src8), "{6}"(size) : "memory" : "volatile"
            };
            size = size.saturating_sub(64);
        }

        while size > 15 {
            vfpu_asm! {
                lv_q C000, 0(a1);
                sv_q C000, 0(a0);
                //.mips "addiu $$a2, $$a2, -16";
                .mips "addiu $$a1, $$a1, 16";
                .mips "addiu $$a0, $$a0, 16";
                : : "{4}"(dst8), "{5}"(src8), "{6}"(size) : "memory" : "volatile"
            }
            size = size.saturating_sub(16);
        }

        let mut dst32 = dst8 as *mut u32;
        let mut src32 = src8 as *const u32;

        while size > 3 {
            *dst32 = *src32;
            dst32 = dst32.add(1);
            src32 = src32.add(1);
            size = size.saturating_sub(4);
        }

        while size > 0 {
            *dst8 = *src8;
            dst8 = dst8.add(1);
            src8 = src8.add(1);
            size = size.saturating_sub(1);
        }
        dst8
    } else {
         panic!("Unaligned");
    }
}

