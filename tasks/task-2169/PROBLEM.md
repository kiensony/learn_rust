# task-2169 — Sorted insertion moves · longest run

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Instrument an algorithm on a stream of keys. Observe the number of older elements shifted right when each key is inserted stably after any equals in a sorted vector. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn sorted_insertion_moves_longest_run(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `sorted_insertion_moves_longest_run(&[])` gives `0`.
- `sorted_insertion_moves_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2169/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2169`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
