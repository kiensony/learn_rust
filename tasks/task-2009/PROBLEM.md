# task-2009 — Polled lengths · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Create one ready future per byte and poll each exactly once. Observe the ready payload, the byte itself.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use a pinned future and an explicit polling context. During review, point to the operation that enforces its contract.

```rust
pub fn polled_lengths_longest_run(input: &[u8]) -> u64
```

## Public examples

- `polled_lengths_longest_run(&[])` gives `0`.
- `polled_lengths_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2009/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2009`.

[Module reading](https://doc.rust-lang.org/std/future/)
