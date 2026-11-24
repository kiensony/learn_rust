# task-193 — Alarm band · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Sound an alarm below 8 or above 10.

```rust
pub fn alarm_band_8(start: u16, end: u16) -> bool
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `alarm_band_8(4, 0)` gives `true`.
- `alarm_band_8(10, 1)` gives `false`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-193/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-193`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
