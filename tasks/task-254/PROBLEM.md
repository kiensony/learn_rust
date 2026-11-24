# task-254 — Digit sum · limited supply

A request is limited by available stock. Use the smaller count as value. Add all base-10 digits of value. Zero has digit sum zero.

```rust
pub fn digit_sum_10(requested: u8, available: u8) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `digit_sum_10(255, 4)` gives `4`.
- `digit_sum_10(255, 0)` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-254/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-254`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
