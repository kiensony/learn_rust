# task-2168 — Sorted insertion moves · steps

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Instrument an algorithm on a stream of keys. Observe the number of older elements shifted right when each key is inserted stably after any equals in a sorted vector. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Return the absolute differences between each adjacent pair, in order. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn sorted_insertion_moves_steps(input: &[u8]) -> Vec<u32>
```

## Public examples

- `sorted_insertion_moves_steps(&[])` gives `vec![]`.
- `sorted_insertion_moves_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2168/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2168`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
