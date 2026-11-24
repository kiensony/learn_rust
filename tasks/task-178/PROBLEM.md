# task-178 — Priority fee · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Return 0 for zero, 3 for a positive value through 5, and twice 3 otherwise.

```rust
pub fn priority_fee_3(first: u16, second: u16) -> u8
```

Implement the public function in `src/lib.rs`.

## Public examples

- `priority_fee_3(4, 0)` gives `3`.
- `priority_fee_3(13, 3)` gives `6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-178/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-178`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
