# task-253 — Running deposits · limited supply

A request is limited by available stock. Use the smaller count as value. Deposit 10 more coins each day than the day before, starting with 10 on day one. Return total for value days.

```rust
pub fn running_deposits_10(requested: u8, available: u8) -> u32
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `running_deposits_10(255, 4)` gives `100`.
- `running_deposits_10(13, 14)` gives `910`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-253/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-253`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
