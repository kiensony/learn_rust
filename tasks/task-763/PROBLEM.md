# task-763 — Drain the waiting room in arrival order

Queue each arrival in a VecDeque. Serve at most slots entries from the front, returning their values in service order. Duplicate tickets represent distinct arrivals.

```rust
pub fn served(arrivals: &[u8], slots: usize) -> Vec<u8>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `served(&[2, 1, 2], 2)` gives `vec![2, 1]`.
- `served(&[4, 5], 3)` gives `vec![4, 5]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-763/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-763`.

[Module reading](https://doc.rust-lang.org/std/collections/)
