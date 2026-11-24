# task-194 — Even batch · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Require an even value at least 8.

```rust
pub fn even_batch_8(start: u16, end: u16) -> bool
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `even_batch_8(4, 0)` gives `false`.
- `even_batch_8(13, 1)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-194/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-194`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
