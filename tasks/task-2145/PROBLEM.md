# task-2145 — First occurrences · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A scheduler receives one u8 key per entry. Observe the sequence in which previously unseen keys first arrive. Explain the cost of inserting and extracting the next item.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use the standard library's HashSet membership and input-order iteration. During review, point to the operation that enforces its contract.

```rust
pub fn first_occurrences_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `first_occurrences_changes(&[])` gives `vec![]`.
- `first_occurrences_changes(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2145/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2145`.

[Module reading](https://doc.rust-lang.org/std/collections/)
