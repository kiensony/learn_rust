# task-2870 — Big endian registers · reverse

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Decode a portable register image for a host and WebAssembly consumer. Each record occupies two bytes. Ignore a trailing unpaired byte. Observe the big-endian unsigned 16-bit reading per complete record. Verify with cargo check --target wasm32-unknown-unknown after installing the target, and explain the byte order contract.

Return the observations in reverse order. Retain duplicate observations. Use fixed-width integer operations independent of host byte order. During review, point to the operation that enforces its contract.

```rust
pub fn big_endian_registers_reverse(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `big_endian_registers_reverse(&[])` gives `vec![]`.
- `big_endian_registers_reverse(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2870/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2870`.

[Module reading](https://doc.rust-lang.org/rustc/platform-support.html)
