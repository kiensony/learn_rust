# task-148 — Digit place · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Read the base-8 digit in the second position from the right.

```rust
pub fn digit_place_8(start: u16, end: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `digit_place_8(4, 0)` gives `0`.
- `digit_place_8(13, 1)` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-148/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-148`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
