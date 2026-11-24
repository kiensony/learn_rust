# task-097 — Reverse a reading · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Subtract value from 65535, then divide the difference by 8 using integer division.

```rust
pub fn reverse_a_reading_8(start: u16, end: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `reverse_a_reading_8(4, 0)` gives `8191`.
- `reverse_a_reading_8(13, 1)` gives `8190`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-097/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-097`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
