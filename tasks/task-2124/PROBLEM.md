# task-2124 — Queue rotation · peaks

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A scheduler receives one u8 key per entry. Observe the FIFO order after moving the front element to the back exactly once, when present. Explain the cost of inserting and extracting the next item.

Keep the first observation and every later one strictly above all earlier observations. Use the standard library's VecDeque. During review, point to the operation that enforces its contract.

```rust
pub fn queue_rotation_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `queue_rotation_peaks(&[])` gives `vec![]`.
- `queue_rotation_peaks(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2124/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2124`.

[Module reading](https://doc.rust-lang.org/std/collections/)
