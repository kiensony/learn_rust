# task-2895 — Signed sensor magnitude · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Decode a portable register image for a host and WebAssembly consumer. Each record occupies two bytes. Ignore a trailing unpaired byte. Observe the absolute magnitude of the little-endian signed 16-bit reading per complete record. Verify with cargo check --target wasm32-unknown-unknown after installing the target, and explain the byte order contract.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use fixed-width integer operations independent of host byte order. During review, point to the operation that enforces its contract.

```rust
pub fn signed_sensor_magnitude_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `signed_sensor_magnitude_changes(&[])` gives `vec![]`.
- `signed_sensor_magnitude_changes(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2895/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2895`.

[Module reading](https://doc.rust-lang.org/rustc/platform-support.html)
