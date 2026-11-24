# task-246 — Doubling steps · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Start at 8. Count doublings until you reach or exceed value.

```rust
pub fn doubling_steps_8(start: u8, end: u8) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `doubling_steps_8(4, 0)` gives `0`.
- `doubling_steps_8(13, 1)` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-246/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-246`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
