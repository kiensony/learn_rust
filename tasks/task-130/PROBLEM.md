# task-130 — Fractional fee · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Charge 3 percent of value. Discard any fraction of a cent.

```rust
pub fn fractional_fee_3(first: u16, second: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `fractional_fee_3(4, 0)` gives `0`.
- `fractional_fee_3(65535, 4)` gives `1966`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-130/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-130`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
