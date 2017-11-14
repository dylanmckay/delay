//! A 10-cycle atom.

/// The number of CPU cycles it takes to execute the atom.
pub const CYCLES_PER_ATOM: u32 = 25;

/// Gets the number of atoms to be used to delay
/// for a given number of milliseconds.
///
/// The number of atoms, when executed, should at least
/// cover the entire duration denoted by `millis`.
pub const fn iterations_required(cycles: u32) -> u32 {
    // N.B. Integer division is truncating and so we will 
    cycles / CYCLES_PER_ATOM
}

/// Runs a small delay of exactly 21 cycles, not including the call instruction.
///
/// Note that `1` can be added to the cycles for each instruction
/// if the system RAM is external.
#[naked]
#[allow(dead_code)] // this is only called via inline assembly.
#[no_mangle]
pub extern fn delay_21_cycles() {
    unsafe { asm!("nop" :::: "volatile") } // 1 cycle
    unsafe { asm!("nop" :::: "volatile") } // 2 cycles
    unsafe { asm!("nop" :::: "volatile") } // 3 cycles
    unsafe { asm!("nop" :::: "volatile") } // 4 cycles
    unsafe { asm!("nop" :::: "volatile") } // 5 cycles
    unsafe { asm!("nop" :::: "volatile") } // 6 cycles
    unsafe { asm!("nop" :::: "volatile") } // 7 cycles
    unsafe { asm!("nop" :::: "volatile") } // 8 cycles
    unsafe { asm!("nop" :::: "volatile") } // 9 cycles
    unsafe { asm!("nop" :::: "volatile") } // 10 cycles
    unsafe { asm!("nop" :::: "volatile") } // 11 cycles
    unsafe { asm!("nop" :::: "volatile") } // 12 cycles
    unsafe { asm!("nop" :::: "volatile") } // 13 cycles
    unsafe { asm!("nop" :::: "volatile") } // 14 cycles
    unsafe { asm!("nop" :::: "volatile") } // 15 cycles
    unsafe { asm!("nop" :::: "volatile") } // 16 cycles
    unsafe { asm!("nop" :::: "volatile") } // 17 cycles

    // A `ret` will always be 4 cycles if internal RAM is used.
    // If external RAM was setup, this would take 5 cycles.
    // We could omit this as the compiler will always emit a basic
    // ret instruction, but we do it explicitly to be 100% sure.
    unsafe { asm!("ret") } // 21 cycles
}

