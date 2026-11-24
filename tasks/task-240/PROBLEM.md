# task-240 — Count chimes · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Count positive multiples of 8 up to and including value.

```rust
pub fn count_chimes_8(start: u8, end: u8) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `count_chimes_8(4, 0)` gives `0`.
- `count_chimes_8(13, 1)` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-240/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-240`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
