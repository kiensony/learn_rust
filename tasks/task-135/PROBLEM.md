# task-135 — Top up · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Return how many units to add to reach a multiple of 5. A full group needs zero.

```rust
pub fn top_up_5(issued: u16, returned: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `top_up_5(4, 0)` gives `1`.
- `top_up_5(13, 1)` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-135/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-135`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
