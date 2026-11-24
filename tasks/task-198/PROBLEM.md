# task-198 — Priority fee · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Return 0 for zero, 8 for a positive value through 10, and twice 8 otherwise.

```rust
pub fn priority_fee_8(start: u16, end: u16) -> u8
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `priority_fee_8(4, 0)` gives `8`.
- `priority_fee_8(13, 1)` gives `16`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-198/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-198`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
