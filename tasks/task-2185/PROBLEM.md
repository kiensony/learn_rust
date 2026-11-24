# task-2185 — Early exit probes · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Instrument an algorithm on a stream of keys. Observe the number of earlier entries inspected left-to-right, stopping at the first that is strictly greater than the current key, or after all earlier entries. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn early_exit_probes_changes(input: &[u8]) -> Vec<u32>
```

## Public examples

- `early_exit_probes_changes(&[])` gives `vec![]`.
- `early_exit_probes_changes(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2185/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2185`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
