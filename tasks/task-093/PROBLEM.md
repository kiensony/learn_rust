# task-093 — Complete rows · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Arrange value tiles in rows of 8. Return the number of complete rows.

```rust
pub fn complete_rows_8(start: u16, end: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `complete_rows_8(4, 0)` gives `0`.
- `complete_rows_8(13, 1)` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-093/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-093`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
