# task-102 — Opening balance · limited supply

A request is limited by available stock. Use the smaller count as value. The register starts with 10 tokens. Add the delivered tokens.

```rust
pub fn opening_balance_10(requested: u16, available: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `opening_balance_10(65535, 4)` gives `14`.
- `opening_balance_10(13, 14)` gives `23`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-102/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-102`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
