# task-153 — Round down · limited supply

A request is limited by available stock. Use the smaller count as value. Return the greatest multiple of 10 no larger than value.

```rust
pub fn round_down_10(requested: u16, available: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `round_down_10(65535, 4)` gives `0`.
- `round_down_10(13, 14)` gives `10`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-153/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-153`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
