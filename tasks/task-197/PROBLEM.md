# task-197 — Three shelves · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Return 0 below 8, 1 from 8 through 10 inclusive, and 2 above 10.

```rust
pub fn three_shelves_8(start: u16, end: u16) -> u8
```

Implement the public function in `src/lib.rs`.

## Public examples

- `three_shelves_8(4, 0)` gives `0`.
- `three_shelves_8(13, 1)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-197/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-197`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
