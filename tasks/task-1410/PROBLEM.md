# task-1410 — Millisecond remainder · reverse

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the milliseconds remaining after whole seconds.

Return the observations in reverse order. Retain duplicate observations. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn millisecond_remainder_reverse(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `millisecond_remainder_reverse(&[])` gives `vec![]`.
- `millisecond_remainder_reverse(&[4])` gives `vec![68u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1410/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1410`.

[Module reading](https://doc.rust-lang.org/std/time/)
