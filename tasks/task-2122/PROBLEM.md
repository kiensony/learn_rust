# task-2122 — Queue rotation · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A scheduler receives one u8 key per entry. Observe the FIFO order after moving the front element to the back exactly once, when present. Explain the cost of inserting and extracting the next item.

Return the sum of all observations, widening each one before adding. Use the standard library's VecDeque. During review, point to the operation that enforces its contract.

```rust
pub fn queue_rotation_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `queue_rotation_total(&[])` gives `0`.
- `queue_rotation_total(&[4])` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2122/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2122`.

[Module reading](https://doc.rust-lang.org/std/collections/)
