# task-2872 — Flag register · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Decode a portable register image for a host and WebAssembly consumer. Each record occupies two bytes. Ignore a trailing unpaired byte. Observe the bits enabled in both the flags byte and mask byte per complete record. Verify with cargo check --target wasm32-unknown-unknown after installing the target, and explain the byte order contract.

Return the sum of all observations, widening each one before adding. Use fixed-width integer operations independent of host byte order. During review, point to the operation that enforces its contract.

```rust
pub fn flag_register_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `flag_register_total(&[])` gives `0`.
- `flag_register_total(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2872/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2872`.

[Module reading](https://doc.rust-lang.org/rustc/platform-support.html)
