# task-196 — Override code · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Allow 8, 10, and no other code.

```rust
pub fn override_code_8(start: u16, end: u16) -> bool
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `override_code_8(4, 0)` gives `false`.
- `override_code_8(8, 0)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-196/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-196`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
