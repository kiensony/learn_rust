# task-145 — Top up · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Return how many units to add to reach a multiple of 8. A full group needs zero.

```rust
pub fn top_up_8(start: u16, end: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `top_up_8(4, 0)` gives `4`.
- `top_up_8(0, 2)` gives `6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-145/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-145`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
