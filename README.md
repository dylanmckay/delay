# delay

Busy-loop compile-time delays for Rust on AVR.

## Behind-the-hood

Behind the hood, this crate uses a combination of macros and `const fn`s
that generates code that takes a busy loop that executes for specified periods of time.

Unlike AVR-GCC's [`util/delay.h` library](http://www.nongnu.org/avr-libc/user-manual/group__util__delay.html#details), this crate does not rely on optimizations being enabled (causing silent bugs otherwise) - everything is always computed at compile time through `const fn`s.

## Example

More examples can be found in the [examples/](examples/) folder.

```rust
#[macro_use]
extern crate delay;

fn main() {
    loop {
        do_something();

        delay_ms!(1000);

        do_something();

        delay_ms!(1000);
    }
}
```
