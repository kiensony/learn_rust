# task-242 — Skip reserved seats · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Count seats from 1 through value, skipping every Nth seat.

```rust
pub fn skip_reserved_seats_8(start: u8, end: u8) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `skip_reserved_seats_8(4, 0)` gives `4`.
- `skip_reserved_seats_8(13, 1)` gives `11`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-242/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-242`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
