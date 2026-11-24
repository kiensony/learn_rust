# task-152 — Price in cents · limited supply

A request is limited by available stock. Use the smaller count as value. At 10 cents per unit, compute the total price as a u32.

```rust
pub fn price_in_cents_10(requested: u16, available: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `price_in_cents_10(65535, 4)` gives `40`.
- `price_in_cents_10(13, 14)` gives `130`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-152/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-152`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
