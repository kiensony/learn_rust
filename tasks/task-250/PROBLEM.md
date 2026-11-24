# task-250 — Count chimes · limited supply

A request is limited by available stock. Use the smaller count as value. Count positive multiples of 10 up to and including value.

```rust
pub fn count_chimes_10(requested: u8, available: u8) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `count_chimes_10(255, 4)` gives `0`.
- `count_chimes_10(13, 14)` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-250/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-250`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
