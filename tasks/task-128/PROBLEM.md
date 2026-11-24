# task-128 — Digit place · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Read the base-3 digit in the second position from the right.

```rust
pub fn digit_place_3(first: u16, second: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `digit_place_3(4, 0)` gives `1`.
- `digit_place_3(13, 3)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-128/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-128`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
