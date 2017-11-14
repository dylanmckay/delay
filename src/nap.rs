//! Utilities for sleeping the CPU.

use util;

/// The number of CPU cycles used by the nap routine itself.
pub const NAP_STANDALONE_CYCLES: u32 = 21;
/// The number of CPU cycles it takes to actually invoke the nap routine.
pub const CALL_INSTRUCTION_CYCLES: u32 = 4;

/// The number of CPU cycles it takes to execute one nap.
pub const CYCLES_PER_NAP: u32 = NAP_STANDALONE_CYCLES + CALL_INSTRUCTION_CYCLES;

/// Gets the number of iterations to be used to delay
/// for a given number of milliseconds.
///
/// The number of naps, when executed, should at least
/// cover the entire duration denoted by `millis`.
pub const fn naps_required(cycles: u32) -> u32 {
    // Round up so we never underestimate the time. At the very worst,
    // we will overestimate it by clock_cycles(nap)-1 which is small.
    util::division_ceil(cycles, CYCLES_PER_NAP)
}

/// Runs a small delay of exactly 21 cycles, not including the call instruction.
///
/// Note that `1` can be added to the cycles for each instruction
/// if the system RAM is external.
#[naked]
#[allow(dead_code)] // this is only called via inline assembly.
#[no_mangle]
pub extern fn __avr_rust_perform_nap() {
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

