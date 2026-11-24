# task-132 — Price in cents · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. At 5 cents per unit, compute the total price as a u32.

```rust
pub fn price_in_cents_5(issued: u16, returned: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `price_in_cents_5(4, 0)` gives `20`.
- `price_in_cents_5(13, 1)` gives `60`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-132/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-132`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
