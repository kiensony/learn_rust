# task-2162 — Sorted insertion moves · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Instrument an algorithm on a stream of keys. Observe the number of older elements shifted right when each key is inserted stably after any equals in a sorted vector. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Return the sum of all observations, widening each one before adding. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn sorted_insertion_moves_total(input: &[u8]) -> u64
```

## Public examples

- `sorted_insertion_moves_total(&[])` gives `0`.
- `sorted_insertion_moves_total(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2162/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2162`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
