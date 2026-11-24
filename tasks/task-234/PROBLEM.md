# task-234 — Digit sum · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Add all base-5 digits of value. Zero has digit sum zero.

```rust
pub fn digit_sum_5(issued: u8, returned: u8) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `digit_sum_5(4, 0)` gives `4`.
- `digit_sum_5(0, 2)` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-234/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-234`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
