# task-2515 — Decode through a C ABI boundary

Labeled advanced FFI lab. A companion library exports track_sensor_value through the C ABI and expects the repr(C) layout in fixtures/sensor.h. The function returns (reading as i32) multiplied by scale (as i32). Inputs include negative readings, zero scales, and extreme i16 values. Pass the structure by value across the declared boundary, preserving the reading's sign. Explain why a header and Rust representation must agree about width, ordering, and alignment.

```rust
pub fn scaled_reading(reading: i16, scale: u8) -> i32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `scaled_reading(12, 3)` gives `36`.
- `scaled_reading(-5, 4)` gives `-20`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2515/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2515`.

[Module reading](https://doc.rust-lang.org/reference/items/external-blocks.html)
