# task-203 — Alarm band · limited supply

A request is limited by available stock. Use the smaller count as value. Sound an alarm below 10 or above 12.

```rust
pub fn alarm_band_10(requested: u16, available: u16) -> bool
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `alarm_band_10(65535, 4)` gives `true`.
- `alarm_band_10(10, 11)` gives `false`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-203/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-203`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
