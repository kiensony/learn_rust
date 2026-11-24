# task-099 — A full tray · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Return true exactly when value tiles fill a whole number of trays of 8, including zero trays.

```rust
pub fn a_full_tray_8(start: u16, end: u16) -> bool
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `a_full_tray_8(4, 0)` gives `false`.
- `a_full_tray_8(4, 4)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-099/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-099`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
