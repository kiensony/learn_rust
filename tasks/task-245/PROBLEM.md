# task-245 — Digit count · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Count the base-8 digits of value. Zero takes one digit.

```rust
pub fn digit_count_8(start: u8, end: u8) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `digit_count_8(4, 0)` gives `1`.
- `digit_count_8(13, 1)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-245/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-245`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
