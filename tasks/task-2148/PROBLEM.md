# task-2148 — Queue rotation · rolling load

A scheduler receives one u8 key per entry. Observe the FIFO order after moving the front element to the back exactly once, when present. Explain the cost of inserting and extracting the next item.

For each complete sliding window of width observations, return its sum as u64. Consecutive windows overlap by width minus one observations. Width zero and widths greater than the trace length yield an empty vector.

Use the standard library's VecDeque. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn queue_rotation_rolling_load(input: &[u8], width: usize) -> Vec<u64>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `queue_rotation_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1)` gives `vec![6u64, 9u64, 12u64, 21u64, 15u64, 0u64, 26u64, 8u64, 31u64, 3u64]`.
- `queue_rotation_rolling_load(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2148/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2148`.

[Module reading](https://doc.rust-lang.org/std/collections/)
