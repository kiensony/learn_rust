# task-252 — Skip reserved seats · limited supply

A request is limited by available stock. Use the smaller count as value. Count seats from 1 through value, skipping every Nth seat.

```rust
pub fn skip_reserved_seats_10(requested: u8, available: u8) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `skip_reserved_seats_10(255, 4)` gives `4`.
- `skip_reserved_seats_10(13, 14)` gives `12`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-252/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-252`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
