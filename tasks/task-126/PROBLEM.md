# task-126 — Saturating debit · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Remove 3 units from value. Stop at zero.

```rust
pub fn saturating_debit_3(first: u16, second: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `saturating_debit_3(4, 0)` gives `1`.
- `saturating_debit_3(13, 3)` gives `13`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-126/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-126`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
