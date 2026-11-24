# task-1439 — Timer windows · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the number of started 100-millisecond windows, with zero windows for zero duration.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn timer_windows_longest_run(input: &[u8]) -> u64
```

## Public examples

- `timer_windows_longest_run(&[])` gives `0`.
- `timer_windows_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1439/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1439`.

[Module reading](https://doc.rust-lang.org/std/time/)
