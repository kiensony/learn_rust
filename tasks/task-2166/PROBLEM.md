# task-2166 — Sorted insertion moves · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Instrument an algorithm on a stream of keys. Observe the number of older elements shifted right when each key is inserted stably after any equals in a sorted vector. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Count adjacent pairs for which the second observation is strictly smaller. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn sorted_insertion_moves_descending_steps(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `sorted_insertion_moves_descending_steps(&[])` gives `0`.
- `sorted_insertion_moves_descending_steps(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2166/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2166`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
