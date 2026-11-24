# task-247 — Cycle ticks · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. A wheel starts at position zero on a ring of 10 positions. Advance 8 places value times and return its final position.

```rust
pub fn cycle_ticks_8(start: u8, end: u8) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `cycle_ticks_8(4, 0)` gives `2`.
- `cycle_ticks_8(13, 1)` gives `6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-247/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-247`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
