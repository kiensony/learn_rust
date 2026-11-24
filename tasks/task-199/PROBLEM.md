# task-199 — Status lamp · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Return "low" below 8, "full" above 10, and "ready" in between (including the boundaries).

```rust
pub fn status_lamp_8(start: u16, end: u16) -> &'static str
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `status_lamp_8(4, 0)` gives `"low"`.
- `status_lamp_8(13, 1)` gives `"full"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-199/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-199`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
