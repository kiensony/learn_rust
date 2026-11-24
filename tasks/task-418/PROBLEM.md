# task-418 — Cap readings in place

Change each reading above the ceiling to the ceiling in the caller’s slice. Return how many values changed. A reading already at the ceiling stays unchanged.

```rust
pub fn cap(readings: &mut [u16], ceiling: u16) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `{ let mut data = [3, 8]; let changed = cap(&mut data, 5); (data, changed) }` gives `([3, 5], 1)`.
- `{ let mut data = [9, 9, 1]; let changed = cap(&mut data, 9); (data, changed) }` gives `([9, 9, 1], 0)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-418/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-418`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
