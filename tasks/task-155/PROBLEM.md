# task-155 — Top up · limited supply

A request is limited by available stock. Use the smaller count as value. Return how many units to add to reach a multiple of 10. A full group needs zero.

```rust
pub fn top_up_10(requested: u16, available: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `top_up_10(65535, 4)` gives `6`.
- `top_up_10(13, 14)` gives `7`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-155/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-155`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
