# task-224 — Digit sum · two deliveries

Combine two deliveries, capping their sum at 255. Use the combined count as value. Add all base-3 digits of value. Zero has digit sum zero.

```rust
pub fn digit_sum_3(first: u8, second: u8) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `digit_sum_3(4, 0)` gives `2`.
- `digit_sum_3(13, 3)` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-224/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-224`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
