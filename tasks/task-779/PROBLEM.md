# task-779 — Count distinct visitors across sessions

Count distinct visitor IDs across all sessions. Count a visitor once even when they appear in several sessions or repeat within one session.

```rust
pub fn unique(sessions: &[&[u8]]) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `unique(&[&[2,2], &[3,2]])` gives `2`.
- `unique(&[])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-779/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-779`.

[Module reading](https://doc.rust-lang.org/std/collections/)
