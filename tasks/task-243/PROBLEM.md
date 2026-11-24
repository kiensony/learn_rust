# task-243 — Running deposits · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Deposit 8 more coins each day than the day before, starting with 8 on day one. Return total for value days.

```rust
pub fn running_deposits_8(start: u8, end: u8) -> u32
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `running_deposits_8(4, 0)` gives `80`.
- `running_deposits_8(13, 1)` gives `624`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-243/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-243`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
