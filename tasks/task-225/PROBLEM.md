# task-225 — Digit count · two deliveries

Combine two deliveries, capping their sum at 255. Use the combined count as value. Count the base-3 digits of value. Zero takes one digit.

```rust
pub fn digit_count_3(first: u8, second: u8) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `digit_count_3(4, 0)` gives `2`.
- `digit_count_3(13, 3)` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-225/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-225`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
