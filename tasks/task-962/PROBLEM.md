# task-962 — Pair stations with their successor

Collect all adjacent station pairs in traversal order. Consecutive equal IDs still form one leg. Fewer than two stops produce no legs.

```rust
pub fn legs(stops: &[u8]) -> Vec<(u8,u8)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `legs(&[1, 4, 2])` gives `vec![(1, 4), (4, 2)]`.
- `legs(&[7, 7])` gives `vec![(7, 7)]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-962/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-962`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
