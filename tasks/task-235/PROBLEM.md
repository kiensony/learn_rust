# task-235 — Digit count · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Count the base-5 digits of value. Zero takes one digit.

```rust
pub fn digit_count_5(issued: u8, returned: u8) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `digit_count_5(4, 0)` gives `1`.
- `digit_count_5(13, 1)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-235/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-235`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
