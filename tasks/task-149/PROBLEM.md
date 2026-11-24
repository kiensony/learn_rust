# task-149 — Nearest group · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Round value to the nearest multiple of 8. At the midpoint choose the higher multiple.

```rust
pub fn nearest_group_8(start: u16, end: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `nearest_group_8(4, 0)` gives `8`.
- `nearest_group_8(13, 1)` gives `16`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-149/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-149`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
