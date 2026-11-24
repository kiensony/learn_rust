# task-241 — Sum chimes · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Sum the positive multiples of 8 no greater than value.

```rust
pub fn sum_chimes_8(start: u8, end: u8) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `sum_chimes_8(4, 0)` gives `0`.
- `sum_chimes_8(13, 1)` gives `8`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-241/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-241`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
