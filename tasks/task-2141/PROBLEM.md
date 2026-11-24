# task-2141 — First occurrences · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A scheduler receives one u8 key per entry. Observe the sequence in which previously unseen keys first arrive. Explain the cost of inserting and extracting the next item.

Return the observations in input order, including duplicates. Use the standard library's HashSet membership and input-order iteration. During review, point to the operation that enforces its contract.

```rust
pub fn first_occurrences_trace(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `first_occurrences_trace(&[])` gives `vec![]`.
- `first_occurrences_trace(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2141/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2141`.

[Module reading](https://doc.rust-lang.org/std/collections/)
