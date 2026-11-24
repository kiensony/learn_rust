# task-150 — Fractional fee · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Charge 8 percent of value. Discard any fraction of a cent.

```rust
pub fn fractional_fee_8(start: u16, end: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `fractional_fee_8(4, 0)` gives `0`.
- `fractional_fee_8(65535, 0)` gives `5242`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-150/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-150`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
