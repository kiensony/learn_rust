# task-244 — Digit sum · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Add all base-8 digits of value. Zero has digit sum zero.

```rust
pub fn digit_sum_8(start: u8, end: u8) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `digit_sum_8(4, 0)` gives `4`.
- `digit_sum_8(13, 1)` gives `5`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-244/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-244`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
