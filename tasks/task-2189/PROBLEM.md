# task-2189 — Early exit probes · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Instrument an algorithm on a stream of keys. Observe the number of earlier entries inspected left-to-right, stopping at the first that is strictly greater than the current key, or after all earlier entries. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn early_exit_probes_longest_run(input: &[u8]) -> u64
```

## Public examples

- `early_exit_probes_longest_run(&[])` gives `0`.
- `early_exit_probes_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2189/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2189`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
