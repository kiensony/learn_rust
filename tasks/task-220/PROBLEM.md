# task-220 — Stop reading at a marker

Each element is either a reading or the stop marker 255. Add only readings before the first stop marker. The keyword break leaves a loop immediately. Sum all values when the marker is absent. Inputs contain at most 1000 readings.

```rust
pub fn before_stop(readings: &[u8]) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `before_stop(&[1, 2, 255, 4])` gives `3`.
- `before_stop(&[255, 3])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-220/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-220`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
