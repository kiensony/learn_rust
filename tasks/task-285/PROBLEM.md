# task-285 — Recover a missing return value

The function must return the following room number as u32. The diagnostic reports a type mismatch. Inspect how the final semicolon turns a value-producing expression into a statement that returns the unit value ().

```rust
pub fn next_room(room: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `next_room(1)` gives `2`.
- `next_room(4)` gives `5`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-285/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-285`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
