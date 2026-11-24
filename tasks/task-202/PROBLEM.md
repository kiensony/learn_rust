# task-202 — Comfort zone · limited supply

A request is limited by available stock. Use the smaller count as value. Accept values from 10 through 12 inclusive.

```rust
pub fn comfort_zone_10(requested: u16, available: u16) -> bool
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `comfort_zone_10(65535, 4)` gives `false`.
- `comfort_zone_10(10, 11)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-202/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-202`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
