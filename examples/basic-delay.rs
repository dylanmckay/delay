#![feature(asm, lang_items, unwind_attributes)]

#![no_std]
#![no_main]

#[macro_use]
extern crate delay;

#[no_mangle]
pub extern fn main() {
    delay_ms!(200);
}

// These do not need to be in a module, but we group them here for clarity.
pub mod std {
    #[lang = "eh_personality"]
    #[no_mangle]
    pub unsafe extern "C" fn rust_eh_personality(_state: (), _exception_object: *mut (), _context: *mut ()) -> () {
    }

    #[lang = "panic_fmt"]
    #[unwind]
    pub extern fn rust_begin_panic(_msg: (), _file: &'static str, _line: u32) -> ! {
        loop { }
    }
}

