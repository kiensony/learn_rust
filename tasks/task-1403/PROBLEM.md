# task-1403 — Millisecond remainder · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the milliseconds remaining after whole seconds.

Return Some((minimum, maximum)) or None for an empty trace. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn millisecond_remainder_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `millisecond_remainder_bounds(&[])` gives `None`.
- `millisecond_remainder_bounds(&[4])` gives `Some((68u32, 68u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1403/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1403`.

[Module reading](https://doc.rust-lang.org/std/time/)
