# task-141 — Whole cartons · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Return the fewest cartons of size 8 that can hold value pieces.

```rust
pub fn whole_cartons_8(start: u16, end: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `whole_cartons_8(4, 0)` gives `1`.
- `whole_cartons_8(13, 1)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-141/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-141`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
