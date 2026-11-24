# task-204 — Even batch · limited supply

A request is limited by available stock. Use the smaller count as value. Require an even value at least 10.

```rust
pub fn even_batch_10(requested: u16, available: u16) -> bool
```

Implement the public function in `src/lib.rs`.

## Public examples

- `even_batch_10(65535, 4)` gives `false`.
- `even_batch_10(10, 11)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-204/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-204`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
