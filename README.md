Feather M0 LoRa with Ultimate GPS Featherwing - rs
====================================================

This repo contains Carsten's experiments with a GPS/LoRa device, with firmware written in Rust using the RTIC framework.

Build with `cargo build`, load onto Feather M0 in UF2 mode with `cargo run`. The runner is the `hf2` tool installed with `cargo install hf2-cli`. Be sure the [appropriate udev rules](https://crates.io/crates/cargo-hf2) are in place. The bootloader must be updated to support UF2.

The current implementation uses the `systick-monotonic` crate and M0's systick timer to schedule software tasks using the RTC interrupt.

Work-in-progress...
* There is a `print` task that receives a `&'static str` and prints it to the console.
* There is a `hearbeat` task that prints a message every 2 seconds to the serial port.
* There is a `blink` task that blinks the on-board LED n times.