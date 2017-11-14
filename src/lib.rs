//! AVR compile-time delays.
//!
//! # Design
//!
//! Delay loops are built out of looped executions of a fixed-time 'nap' function.
//!
//! In order to calculate the number of naps needed to cover a delay, we first
//! figure out the number of CPU cycles that cover the delay. Once we know that, we
//! can extrapolate the minimum number of naps  so that when executed sequentially
//! the entire delay is made.
//!
//! # Taking into account errors caused by loop instructions themselves
//!
//! There is a complexity added because the looping code itself actully uses CPU cycles.
//! This means that every iteration of the nap function runs a few extra instructions,
//! and makes the delay cycle takea bit longer. This is why when we perform delays,
//! we must take care to take the clock cycle counts out of the inline
//! assembly in the loop code itself when computing how many naps need to be taken.
//!
//! In order to account for the looping code, `const fn`s calculate how many clock
//! cycles will be added due to the loop instructions, potentially subtracting one
//! or more naps to bring the actual execution time to within 25 clock cycles of the
//! actual time.
//!
//! # Appendix
//!
//! ## Timings of call instructions:
//!
//! | Instruction | Cycles |
//! | ----------- | ------ |
//! | `rcall`     | 3
//! | `call`      | 4

#![feature(asm, const_fn, naked_functions)]

#![no_std]

pub mod nap;
mod util;

/// The CPU frequency.
// FIXME: let clock frequency be configurable somehow.
const CYCLES_PER_SECOND: u32 = 16_000_000;
/// The number of clock cycles per microsecond.
const CYCLES_PER_MICROS: u32 = CYCLES_PER_SECOND / 1_000_000;

/// The number of cycles a single nap takes to run if it is not
/// the last iteration in the execution (where the predicate is false).
const SUCCESSFUL_NAP_CYCLES: u32 = 16;
/// The number of cycles spent executing the final redundant iteration
/// of the loop where the loop condition is false.
const FINAL_NAP_CYCLES: u32 = 7;

/// Gets the number of cycles that are executed in a number of microseconds.
pub const fn cycle_count_micros(micros: u32) -> u32 {
    micros * CYCLES_PER_MICROS
}

/// Calculates how many cycles are wasted inside the looping code.
const fn extraneous_cycles_from_looping(nap_count: u32) -> u32 {
    nap_count * SUCCESSFUL_NAP_CYCLES + // these cycles will run once for each atom
        FINAL_NAP_CYCLES // these cycles will be spent checking the failing loop condition in final iteration.
}

pub const fn actual_naps(cycle_count: u32) -> u32 {
    // N.B. Integer divsion in truncating and so we will automatically round down.
    // This is important because we don't want to oversubtract atoms because
    // we should always delay for AT LEAST the expected duration.
    //
    // Note that the naps_required call has not been moved into a `let` because
    // it is not supported by the compiler as of 2017-11-15.
    nap::naps_required(cycle_count - extraneous_cycles_from_looping(nap::naps_required(cycle_count)))
}

#[naked]
#[inline(never)]
#[no_mangle]
#[allow(unused_variables)] // We refer to arguments directly by registers.
pub fn nap(nap_count: u32) {
    unsafe {
        asm!(".start:");

        // Check if `nap_count` is zero, return if so.
        // This always runs each iteration.
        // Runtime:
        //     - 6 cycles if there are more iterations after this one
        //     - 7 cycles if this is the last iteration..
        {
            asm!("cpi r20, 0"); // 1-cycle
            asm!("eor r0, r0 ; Clear the zero register if it hasn't been already."); // 1-cycle
            asm!("cpc r21, r0"); // 1-cycle
            asm!("cpc r22, r0"); // 1-cycle
            asm!("cpc r23, r0"); // 1-cycle
            asm!("breq .done ; Return if there are no more atoms left"); // 1-cycle if false, two if true.
        }

        // Delay for an atom.
        // Runtime: 4 cycles.
        asm!("call __avr_rust_perform_nap"); // 4-cycles

        // Decrement nap_count.
        // Runtime: 4 cycles.
        {
            // Subtract one from the atom count.
            asm!("subi r20, 1"); // 1-cycle
            asm!("sbci r21, 0"); // 1-cycle
            asm!("sbci r22, 0"); // 1-cycle
            asm!("sbci r23, 0"); // 1-cycle
        }

        asm!("rjmp .start"); // 2-cycles

        asm!(".done:");
        asm!("ret");
    }
}

/// Delays execution for a specified number of cycles.
#[macro_export]
macro_rules! delay_cycles {
    ($cycle_count:expr) => {
        {
            // Place the const fn into a const to force CTFE.
            const CYCLE_COUNT: u32 = $cycle_count;
            const NAP_COUNT: u32 = $crate::actual_naps(CYCLE_COUNT);

            $crate::nap(NAP_COUNT);
        }
    }
}

/// Delays execution for a specified number of microseconds.
#[macro_export]
macro_rules! delay_us {
    ($microseconds:expr) => {
        delay_cycles!($crate::cycle_count_micros($microseconds))
    };
}

/// Delays execution for a specified number of milliseconds.
#[macro_export]
macro_rules! delay_ms {
    ($milliseconds:expr) => {
        delay_us!($milliseconds * 1000)
    };
}

