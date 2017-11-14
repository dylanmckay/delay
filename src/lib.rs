//! AVR delays.
//!
//! # Design
//!
//! Delay loops are built out of looped calls to 'delay atoms'.
//! An atom is simply a function which delays for a predetermined
//! number of cycles.
//!
//! In order to calculate the number of atoms needed to cover a delay, we first
//! figure out the number of cycles that cover the delay. Once we know that, we
//! can extrapolate the minimum number of atoms executed sequentially so that
//! the entire delay is made.
//!
//! The problem is that the looping code itself actully uses cycles. This means
//! that every iteration runs a few instructions and makes the delay cycle take
//! a bit longer.
//!
//! In order to account for the looping code, we figure out how many cycles
//! are wasted, deduce how many atoms that corresponds to, any then simply reduces
//! the initial delay by this amount.
//!
//! Note that the atom must have more cycles than the loop because otherwise the loop
//! would take longer than the atom and we could never account for the error.
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

pub mod atoms;

/// The CPU frequency.
const CYCLES_PER_SECOND: u32 = 16_000_000;
const CYCLES_PER_MICROS: u32 = CYCLES_PER_SECOND / 1_000_000;

/// The number of cycles a single iteration of delay_atom_iterations
/// takes to run if it is not the last iteration in the execution.
const ATOM_DELAY_SUCCESSFUL_ITERATION_CYCLES: u32 = 16;
/// The number of cycles spent executing the final redundant iteration
/// of the loop where the loop condition is false.
const ATOM_DELAY_FINAL_ITERATION_CYCLES: u32 = 7;

/// Gets the number of cycles that are executed in a number of microseconds.
pub const fn cycle_count_micros(micros: u32) -> u32 {
    micros * CYCLES_PER_MICROS
}

/// Calculates how many cycles are wasted inside the looping code.
const fn extraneous_cycles_from_looping(iteration_count: u32) -> u32 {
    iteration_count * ATOM_DELAY_SUCCESSFUL_ITERATION_CYCLES + // these cycles will run once for each atom
        ATOM_DELAY_FINAL_ITERATION_CYCLES // these cycles will be spent checking the failing loop condition in final iteration.
}

/// Calculates how many extra iterations can be ran due to the looping code using cycles itself.
const fn extraneous_iterations_from_looping(iteration_count: u32) -> u32 {
    extraneous_cycles_from_looping(iteration_count) / atoms::twenty_five_cycles::CYCLES_PER_ATOM
}

pub const fn actual_iterations(iteration_count: u32) -> u32 {
    // N.B. Integer divsion in truncating and so we will automatically round down.
    // This is important because we don't want to oversubtract atoms because
    // we should always delay for AT LEAST the expected duration.
    iteration_count - extraneous_iterations_from_looping(iteration_count)
}

#[naked]
#[inline(never)]
#[no_mangle]
#[allow(unused_variables)] // We refer to arguments directly by registers.
pub fn delay_atom_iterations(iteration_count: u32) {
    unsafe {
        asm!(".iteration:");

        // Check if `iteration_count` is zero, return if so.
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
        asm!("call delay_21_cycles"); // 4-cycles

        // Decrement iteration_count.
        // Runtime: 4 cycles.
        {
            // Subtract one from the atom count.
            asm!("subi r20, 1"); // 1-cycle
            asm!("sbci r21, 0"); // 1-cycle
            asm!("sbci r22, 0"); // 1-cycle
            asm!("sbci r23, 0"); // 1-cycle
        }

        asm!("rjmp .iteration"); // 2-cycles

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
            const ITERATIONS_CONSERVATIVE: u32 = $crate::atoms::twenty_five_cycles::iterations_required(CYCLE_COUNT);
            const ITERATIONS: u32 = $crate::actual_iterations(ITERATIONS_CONSERVATIVE);

            $crate::delay_atom_iterations(ITERATIONS);
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

