# task-173 — Alarm band · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Sound an alarm below 3 or above 5.

```rust
pub fn alarm_band_3(first: u16, second: u16) -> bool
```

Implement the public function in `src/lib.rs`.

## Public examples

- `alarm_band_3(4, 0)` gives `false`.
- `alarm_band_3(13, 3)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-173/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-173`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
