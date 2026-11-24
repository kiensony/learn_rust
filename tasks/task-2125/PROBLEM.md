# task-2125 — Queue rotation · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A scheduler receives one u8 key per entry. Observe the FIFO order after moving the front element to the back exactly once, when present. Explain the cost of inserting and extracting the next item.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use the standard library's VecDeque. During review, point to the operation that enforces its contract.

```rust
pub fn queue_rotation_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `queue_rotation_changes(&[])` gives `vec![]`.
- `queue_rotation_changes(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2125/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2125`.

[Module reading](https://doc.rust-lang.org/std/collections/)
