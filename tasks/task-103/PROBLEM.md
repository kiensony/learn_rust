# task-103 — Complete rows · limited supply

A request is limited by available stock. Use the smaller count as value. Arrange value tiles in rows of 10. Return the number of complete rows.

```rust
pub fn complete_rows_10(requested: u16, available: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `complete_rows_10(65535, 4)` gives `0`.
- `complete_rows_10(13, 14)` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-103/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-103`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
