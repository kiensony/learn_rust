# task-2119 — Priority removal · longest run

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A scheduler receives one u8 key per entry. Observe the priority queue's removal order, largest key first, including duplicates. Explain the cost of inserting and extracting the next item.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use the standard library's BinaryHeap. During review, point to the operation that enforces its contract.

```rust
pub fn priority_removal_longest_run(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `priority_removal_longest_run(&[])` gives `0`.
- `priority_removal_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2119/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2119`.

[Module reading](https://doc.rust-lang.org/std/collections/)
