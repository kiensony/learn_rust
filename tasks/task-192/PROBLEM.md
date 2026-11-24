# task-192 — Comfort zone · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Accept values from 8 through 10 inclusive.

```rust
pub fn comfort_zone_8(start: u16, end: u16) -> bool
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `comfort_zone_8(4, 0)` gives `false`.
- `comfort_zone_8(10, 1)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-192/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-192`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
