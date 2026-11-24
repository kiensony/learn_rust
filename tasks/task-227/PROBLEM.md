# task-227 — Cycle ticks · two deliveries

Combine two deliveries, capping their sum at 255. Use the combined count as value. A wheel starts at position zero on a ring of 5 positions. Advance 3 places value times and return its final position.

```rust
pub fn cycle_ticks_3(first: u8, second: u8) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `cycle_ticks_3(4, 0)` gives `2`.
- `cycle_ticks_3(13, 3)` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-227/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-227`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
