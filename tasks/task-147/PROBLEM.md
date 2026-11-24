# task-147 — Fit a byte · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Add 8 to value and return the result capped at 255.

```rust
pub fn fit_a_byte_8(start: u16, end: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `fit_a_byte_8(4, 0)` gives `12`.
- `fit_a_byte_8(13, 1)` gives `20`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-147/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-147`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
