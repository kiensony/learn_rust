# task-087 — Update a changing counter

A delivery changes a running total. The existing program widens both inputs to i32 and updates the binding in a second statement. Use Cargo’s error message to repair the declaration so reassignment is permitted.

```rust
pub fn after_delivery(start: i16, delivered: i16) -> i32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `after_delivery(2, 6)` gives `8`.
- `after_delivery(5, -2)` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-087/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-087`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
