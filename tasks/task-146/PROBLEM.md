# task-146 — Saturating debit · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Remove 8 units from value. Stop at zero.

```rust
pub fn saturating_debit_8(start: u16, end: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `saturating_debit_8(4, 0)` gives `0`.
- `saturating_debit_8(13, 1)` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-146/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-146`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
