# task-1438 — Timer windows · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the number of started 100-millisecond windows, with zero windows for zero duration.

Return the absolute differences between each adjacent pair, in order. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn timer_windows_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `timer_windows_steps(&[])` gives `vec![]`.
- `timer_windows_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1438/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1438`.

[Module reading](https://doc.rust-lang.org/std/time/)
