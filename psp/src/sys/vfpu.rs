use super::types::{
    ScePspFVector2, ScePspIVector2, ScePspFVector3, ScePspIVector3, ScePspFVector4, 
    ScePspIVector4,
};

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
    vector2: *mut ScePspFVector2
) -> *mut ScePspFVector2 {
    (*vector2).x = x;
    (*vector2).y = y;
    vector2 
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
    vector2: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    (*vector2).x = 0.0;
    (*vector2).y = 0.0;
    vector2 
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2NegativeZero(
    vector2: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    (*vector2).y  = f32::from_bits(0x8000_0000);
    (*vector2).x  = f32::from_bits(0x8000_0000); 
    vector2
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Ceil(
    result: *mut ScePspFVector2,
    vector2: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    vfpu_asm! {
        lv_s S000, 0(a1);
        lv_s S001, 4(a1);
        vf2id_p C000, C000, 0;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        : : "{4}"(result), "{5}"(vector2) : "memory" : "volatile"
    }
    result
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Trunc(
    result: *mut ScePspFVector2,
    vector2: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    vfpu_asm! {
        lv_s S000, 0(a1);
        lv_s S001, 4(a1);
        vf2iz_p C000, C000, 0;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        : : "{4}"(result), "{5}"(vector2) : "memory" : "volatile"
    }
    result
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Round(
    result: *mut ScePspFVector2,
    vector2: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    vfpu_asm! {
        lv_s S000, 0(a1);
        lv_s S001, 4(a1);
        vf2in_p C000, C000, 0;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        : : "{4}"(result), "{5}"(vector2) : "memory" : "volatile"
    }
    result
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Floor(
    result: *mut ScePspFVector2,
    vector2: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    vfpu_asm! {
        lv_s S000, 0(a1);
        lv_s S001, 4(a1);
        vf2iu_p C000, C000, 0;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        : : "{4}"(result), "{5}"(vector2) : "memory" : "volatile"
    }
    vector2
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
    result: *mut ScePspFVector2,
    left_addend: *mut ScePspFVector2,
    right_addend: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    (*result).y = (*left_addend).y + (*right_addend).y;
    (*result).x = (*left_addend).x + (*right_addend).x;
    result 
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Sub(
    result: *mut ScePspFVector2,
    minuend: *mut ScePspFVector2,
    subtrahend: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    (*result).y = (*minuend).y - (*subtrahend).y;
    (*result).x = (*minuend).x - (*subtrahend).x;
    result
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
    result: *mut ScePspFVector2,
    vector2: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    (*result).y = 0.0 - (*vector2).y;
    (*result).x = 0.0 - (*vector2).x;
    result
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
    a: *mut ScePspFVector2,
    b: *mut ScePspFVector2,
    max: *mut ScePspFVector2,
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
    a: *mut ScePspFVector2,
    b: *mut ScePspFVector2,
    min: *mut ScePspFVector2,
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
    vector2: *mut ScePspFVector2
) -> f32 {
    ((*vector2).x + (*vector2).y) * 0.5
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
    vector2: *mut ScePspFVector2
) -> bool {
    (*vector2).x.to_bits() | (*vector2).y.to_bits() & 0x7fff_ffff == 0
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
pub unsafe extern "C" fn sceVfpuVector2FaceForward(
    arg1: *mut ScePspFVector2,
    arg2: *mut ScePspFVector2,
    arg3: *mut ScePspFVector2,
    arg4: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    //vfpu_asm! {
        //lv_s S010, 0(a2); 
        //lv_s S011, 4(a2); 
        //lv_s S020, 0(a3);
        //lv_s S021, 4(a3);
        //vdot_p S030, C010, C020;
        //lv_s S000, 0(a1);
        //lv_s S001, 4(a1);
        //vpfxt [0,Y,Z,W];
        //vcmp_s LT, S030, S030;
        //vpfxs [-X,-Y,Z,W];
        //vcmovf_p C000, C000, CC[0];
        //sv_s S000, 0(a0);
        //sv_s S001, 4(a0);
        //: : "{4}"(arg1), "{5}"(arg2), "{6}"(arg3), "{7}"(arg4) : "memory" : "volatile" 
    //}
    arg1
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Reflect(
    arg1: *mut ScePspFVector2,
    arg2: *mut ScePspFVector2,
    arg3: *mut ScePspFVector2,
) -> *mut ScePspFVector2 {
    //vfpu_asm! {
        //lv_s S010, 0(a1);
        //lv_s S011, 4(a1);
        //lv_s S020, 0(a2);
        //lv_s S021, 4(a2);
        //vdot_p S031, C010, C020;
        //vfim_s S030, -2.0;
        //vmul_s S032, S030, S031;
        //vscl_p S032,S030,S031;
        //vadd_p C000, C010, C020;
        //vdot_p S033, C000, C000;
        //vcmp_s EZ, S033, S033;
        //vrsq_s S033, S033;
        //vpfxs [0,Y,Z,W];
        //vcmovt_s S033, S033, CC[0];
        //vpfxd [-1:1, -1:1, M, M];
        //vscl_p C000, C000, S033;
        //sv_s S000, 0(a0);
        //sv_s S001, 4(a0);
    //: : "{4}"(arg1), "{5}"(arg2), "{6}"(arg3) : "memory" : "volatile" 
    //}
    arg1
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector2Refract(
    arg1: *mut ScePspFVector2,
    arg2: *mut ScePspFVector2,
    arg3: *mut ScePspFVector2,
    sp: i32,
) -> *mut ScePspFVector2 {
    //vfpu_asm! {
        //.mips "addiu sp, sp, -0x10";
        //.mips "swc1 $$f12, 0($sp)";
        //lv_s S010, 0(a1);
        //lv_s S011, 4(a1);
        //lv_s S020, 0(a2);
        //lv_s S021, 4(a2);
        //lv_s S030, 0(sp);
        //vdot_p S031, C010, C020;
        //vscl_p C010, C010, S030;
        //vmul_s S032, S030, S030;
        //vmul_s S033, S031, S031;
        //vmul_s S031,S031,S030;
	//vocp_s S033,S033;
	//vmul_s S033,S032,S033;
	//vocp_s S033,S033;
	//vsqrt_s S033,S033;
	//vsub_s S031,S031,S032;
	//vscl_p C020,C020,S031;
	//vadd_p C000,C010,C020;
	//vdot_p S033,C000,C000;
	//vcmp_p ES,C000,C000;
	//vrsq_s S033,S033;
	//vpfxd [-1:1,-1:1,M,M];
	//vscl_p C000,C000,S033;
	//vpfxs [0,0,Z,W];
	//vcmovt_p C000,C000,CC[4];
	//sv_s S000,0(a0);
	//sv_s S001,4(a0);
	//.mips "addiu sp, sp, 0x10";
    //}
    arg1
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Set(
    x: f32,
    y: f32,
    z: f32,
    vector3: *mut ScePspFVector3
) -> *mut ScePspFVector3 {
    (*vector3).x = x;
    (*vector3).y = y;
    (*vector3).z = z; 
    vector3
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Copy(
    dst: *mut ScePspFVector3,
    src: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    let temp1 = (*src).z;
    let temp2 = (*src).y;
    (*dst).x = (*src).x;
    (*dst).y = temp2;
    (*dst).z = temp1;
    dst 
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3PositiveZero(
    vector3: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    (*vector3).x = 0.0;
    (*vector3).y = 0.0;
    (*vector3).z = 0.0;
    vector3 
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3NegativeZero(
    vector3: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    (*vector3).x  = f32::from_bits(0x8000_0000); 
    (*vector3).y  = f32::from_bits(0x8000_0000);
    (*vector3).z  = f32::from_bits(0x8000_0000);
    vector3
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Ceil(
    result: *mut ScePspFVector3,
    vector3: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        lv_s S000, 0(a1);
        lv_s S001, 4(a1);
        lv_s S002, 8(a1);
        vf2id_t C000, C000, 0;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S002, 8(a0);
        : : "{4}"(result), "{5}"(vector3) : "memory" : "volatile"
    }
    result
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Trunc(
    result: *mut ScePspFVector3,
    vector3: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        lv_s S000, 0(a1);
        lv_s S001, 4(a1);
        lv_s S002, 8(a1);
        vf2iz_t C000, C000, 0;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S002, 8(a0);
        : : "{4}"(result), "{5}"(vector3) : "memory" : "volatile"
    }
    result
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Round(
    result: *mut ScePspFVector3,
    vector3: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        lv_s S000, 0(a1);
        lv_s S001, 4(a1);
        lv_s S002, 8(a1);
        vf2in_t C000, C000, 0;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S002, 8(a0);
        : : "{4}"(result), "{5}"(vector3) : "memory" : "volatile"
    }
    result
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Floor(
    result: *mut ScePspFVector3,
    vector3: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        lv_s S000, 0(a1);
        lv_s S001, 4(a1);
        lv_s S002, 8(a1);
        vf2iu_t C000, C000, 0;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S002, 8(a0);
        : : "{4}"(result), "{5}"(vector3) : "memory" : "volatile"
    }
    result
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3FromIVector(
    dst: *mut ScePspFVector3,
    src: *mut ScePspIVector3,
) -> *mut ScePspFVector3 {
    (*dst).z = (*src).z as f32;
    (*dst).y = (*src).y as f32;
    (*dst).x = (*src).x as f32;
    dst
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Add(
    result: *mut ScePspFVector3,
    left_addend: *mut ScePspFVector3,
    right_addend: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    vfpu_asm! {
        lv_s S000, 0(a1);
        lv_s S001, 4(a1);
        lv_s S002, 8(a1);
        lv_s S010, 0(a2);
        lv_s S011, 4(a2);
        lv_s S012, 8(a2);
        vadd_t C000, C000, C010;
        sv_s S000, 0(a0);
        sv_s S001, 4(a0);
        sv_s S002, 8(a0);
        : : "{4}"(result), "{5}"(left_addend), "{6}"(right_addend) : "memory" : "volatile"
    }
    result
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Sub(
    result: *mut ScePspFVector3,
    minuend: *mut ScePspFVector3,
    subtrahend: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    //vfpu_asm! {
        //lv_s S000, 0(a1);
        //lv_s S001, 4(a1);
        //lv_s S002, 8(a1);
        //lv_s S010, 0(a2);
        //lv_s S011, 4(a2);
        //lv_s S012, 8(a2);
        //vsub_t C000, C000, C010;
        //sv_s S000, 0(a0);
        //sv_s S001, 4(a0);
        //sv_s S002, 8(a0);
        //: : "{4}"(result), "{5}"(left_addend), "{6}"(right_addend) : "memory" : "volatile"
    //}
    result
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Mul(
    result: *mut ScePspFVector3,
    multiplicand: *mut ScePspFVector3,
    multiplier: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    //vfpu_asm! {
        //lv_s S000, 0(a1);
        //lv_s S001, 4(a1);
        //lv_s S002, 8(a1);
        //lv_s S010, 0(a2);
        //lv_s S011, 4(a2);
        //lv_s S012, 8(a2);
        //vmul_t C000, C000, C010;
        //sv_s S000, 0(a0);
        //sv_s S001, 4(a0);
        //sv_s S002, 8(a0);
        //: : "{4}"(result), "{5}"(multiplicand), "{6}"(multiplier) : "memory" : "volatile"
    //}
    result
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Div(
    result: *mut ScePspFVector3,
    dividend: *mut ScePspFVector3,
    divisor: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    //vfpu_asm! {
        //lv_s S010, 0(a1);
        //lv_s S011, 4(a1);
        //lv_s S012, 8(a1);
        //lv_s S020, 0(a2);
        //lv_s S021, 4(a2);
        //lv_s S022, 8(a2);
        //vdiv_t C000, C010, C020;
        //sv_s S000, 0(a0);
        //sv_s S001, 4(a0);
        //sv_s S002, 8(a0);
        //: : "{4}"(result), "{5}"(dividend), "{6}"(divisor) : "memory" : "volatile"
    //}
    result
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Neg(
    result: *mut ScePspFVector3,
    vector3: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    (*result).z = 0.0 - (*vector3).z;
    (*result).y = 0.0 - (*vector3).y;
    (*result).x = 0.0 - (*vector3).x;
    result
}

#[no_mangle]
pub unsafe extern "C" fn sceVfpuVector3Abs(
    result: *mut ScePspFVector3,
    vector3: *mut ScePspFVector3,
) -> *mut ScePspFVector3 {
    (*result).z = core::intrinsics::fabsf32((*vector3).z);
    (*result).y = core::intrinsics::fabsf32((*vector3).y);
    (*result).x = core::intrinsics::fabsf32((*vector3).x);
    result
}

//#[no_mangle]
//pub unsafe extern "C" fn sceVfpuVectorLerp(
    //result: *mut ScePspFVector3,
    //arg1: *mut ScePspFVector3,
    //arg2: *mut ScePspFVector3,
    //arg3: f32,
//) -> *mut ScePspFVector3 {
    //vfpu_asm! {
        //.mips "mfc1 $$t0, $f12";
        //mtv t0, S030;
        //lv_s S010, 0(a1);
        //lv_s S011, 4(a1);
        //lv_s S012, 8(a1);
        //lv_s S020, 0(a2);
        //lv_s S021, 4(a2);
        //lv_s S022, 8(a2);
        //vsub_t C000, C020, C010;
        //vscl_t C000, C000, S030;
        //vadd_t C000, C010, C000;
        //sv_s S000, 0(a0);
        //sv_s S001, 4(a0);
        //sv_s S002, 8(a0);
        //: : "{4}"(result), "{5}"(arg1), "{6}"(arg2), "{8}"(arg3) : "memory" : "volatile"
    //}
    //result
//}


#[no_mangle]
pub unsafe extern "C" fn sceVfpuColorAdd(
    result: *mut Color, 
    left_addend: *mut Color,
    right_addend: *mut Color, 
) -> *mut Color {
    vfpu_asm! {
        lv_q C010, 0(a1);
        lv_q C020, 0(a2);
        vadd_q C000, C010, C020;
        sv_q C000, 0(a0);
        : : "{4}"(result), "{5}"(left_addend), "{6}"(right_addend) : "memory" : "volatile"
    }
    result
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

