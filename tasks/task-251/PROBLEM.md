# task-251 — Sum chimes · limited supply

A request is limited by available stock. Use the smaller count as value. Sum the positive multiples of 10 no greater than value.

```rust
pub fn sum_chimes_10(requested: u8, available: u8) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `sum_chimes_10(255, 4)` gives `0`.
- `sum_chimes_10(13, 14)` gives `10`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-251/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-251`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
