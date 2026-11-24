# task-136 — Saturating debit · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Remove 5 units from value. Stop at zero.

```rust
pub fn saturating_debit_5(issued: u16, returned: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `saturating_debit_5(4, 0)` gives `0`.
- `saturating_debit_5(13, 1)` gives `7`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-136/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-136`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
