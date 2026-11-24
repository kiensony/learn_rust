# task-2893 — Signed sensor magnitude · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Decode a portable register image for a host and WebAssembly consumer. Each record occupies two bytes. Ignore a trailing unpaired byte. Observe the absolute magnitude of the little-endian signed 16-bit reading per complete record. Verify with cargo check --target wasm32-unknown-unknown after installing the target, and explain the byte order contract.

Return Some((minimum, maximum)) or None for an empty trace. Use fixed-width integer operations independent of host byte order. During review, point to the operation that enforces its contract.

```rust
pub fn signed_sensor_magnitude_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `signed_sensor_magnitude_bounds(&[])` gives `None`.
- `signed_sensor_magnitude_bounds(&[4])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2893/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2893`.

[Module reading](https://doc.rust-lang.org/rustc/platform-support.html)
