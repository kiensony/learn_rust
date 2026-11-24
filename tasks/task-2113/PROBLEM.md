# task-2113 — Priority removal · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A scheduler receives one u8 key per entry. Observe the priority queue's removal order, largest key first, including duplicates. Explain the cost of inserting and extracting the next item.

Return Some((minimum, maximum)) or None for an empty trace. Use the standard library's BinaryHeap. During review, point to the operation that enforces its contract.

```rust
pub fn priority_removal_bounds(input: &[u8]) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `priority_removal_bounds(&[])` gives `None`.
- `priority_removal_bounds(&[4])` gives `Some((4u32, 4u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2113/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2113`.

[Module reading](https://doc.rust-lang.org/std/collections/)
