# task-142 — Price in cents · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. At 8 cents per unit, compute the total price as a u32.

```rust
pub fn price_in_cents_8(start: u16, end: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `price_in_cents_8(4, 0)` gives `32`.
- `price_in_cents_8(13, 1)` gives `96`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-142/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-142`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
