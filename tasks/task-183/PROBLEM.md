# task-183 — Alarm band · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Sound an alarm below 5 or above 7.

```rust
pub fn alarm_band_5(issued: u16, returned: u16) -> bool
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `alarm_band_5(4, 0)` gives `true`.
- `alarm_band_5(6, 0)` gives `false`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-183/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-183`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
