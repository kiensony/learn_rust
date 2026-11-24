# task-098 — Two counters · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. One dial counts in groups of 8. Return both the complete groups and the remainder.

```rust
pub fn two_counters_8(start: u16, end: u16) -> (u16, u16)
```

Implement the public function in `src/lib.rs`.

## Public examples

- `two_counters_8(4, 0)` gives `(0, 4,)`.
- `two_counters_8(13, 1)` gives `(1, 4,)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-098/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-098`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
