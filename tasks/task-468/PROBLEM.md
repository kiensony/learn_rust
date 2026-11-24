# task-468 — Rotate a three-lamp sequence

Return three lamps rotated left by one position. Preserve duplicates. Use the fixed array length in the type.

```rust
pub fn shift(lamps: [u8; 3]) -> [u8; 3]
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shift([1, 2, 3])` gives `[2, 3, 1]`.
- `shift([9, 0, 2])` gives `[0, 2, 9]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-468/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-468`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)
