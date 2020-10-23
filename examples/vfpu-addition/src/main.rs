#![no_std]
#![no_main]
#![feature(llvm_asm)]

psp::module!("vfpu_test", 1, 1);

use psp::sys::sceVfpuVector4IsZero;
use psp::sys::ScePspFVector4;

fn vfpu_add(a: i32, b: i32) -> i32 {
    let out;

    unsafe {
        psp::vfpu_asm! (
            // Convert `a` to float
            .mips "mtc1 $$a0, $3";
            .mips "cvt.s.w $3, $3";
            .mips "mfc1 $$a0, $3";

            // Convert `b` to float
            .mips "mtc1 $$a1, $3";
            .mips "cvt.s.w $3, $3";
            .mips "mfc1 $$a1, $3";

            // Perform addition
            mtv a0, S000;
            mtv a1, S001;
            vadd_s S000, S000, S001;
            mfv v0, S000;

            // Convert result to `i32`
            .mips "mtc1 $$v0, $3";
            .mips "cvt.w.s $3, $3";
            .mips "mfc1 $$v0, $3";

            : "={2}"(out)
            : "{4}"(a), "{5}"(b)
            : "f"
        );
    }

    out
}

fn psp_main() {
    psp::enable_home_button();

    // Enable the VFPU
    unsafe {
        use psp::sys::{self, ThreadAttributes};
        sys::sceKernelChangeCurrentThreadAttr(0, ThreadAttributes::VFPU);
        // Call a random sceVfpu function so it doesn't get dropped from the binary
        sceVfpuVector4IsZero(&mut ScePspFVector4{x: 0.0, y: 0.0, z: 0.0, w: 0.0 });
    }

    psp::dprintln!("Testing VFPU...");
    psp::dprintln!("VFPU 123 + 4 = {}", vfpu_add(123, 4));
}
