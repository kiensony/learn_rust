# task-2560 — Find an allocation that outlives the batch

A telemetry collector acknowledges an owned batch of samples with its length. Its counts look correct, but the process retains memory after the final batch. Use the Miri interpreter's end-of-process leak check to diagnose this safe Rust program.

## Linux setup

On x86-64 Linux with rustup and a C linker installed, run in this exercise directory. The local toolchain file selects the dated Miri build.

```sh
rustup toolchain install nightly-2025-06-26 --profile minimal --component miri,rust-src
cargo +nightly-2025-06-26 miri setup
cargo +1.88.0 test
cargo +nightly-2025-06-26 miri test --test public acknowledge_one_sample
```

The native test checks the returned number. Miri additionally checks for a leaked allocation at the end of the test executable. Find its allocation site in the report, then trace ownership through `acknowledge`.

Edit `src/lib.rs` so `pub fn acknowledge(sample: Box<[u16]>) -> usize` returns the batch length and releases the allocation by return. Keep the function safe. Empty and repeated batches follow the same rule. Verify the entire suite with `cargo +nightly-2025-06-26 miri test`. Explain how a program can pass its numerical assertions and still fail a leak check. [Miri user guide](https://github.com/rust-lang/miri)
