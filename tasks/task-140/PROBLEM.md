# task-140 — Fractional fee · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Charge 5 percent of value. Discard any fraction of a cent.

```rust
pub fn fractional_fee_5(issued: u16, returned: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `fractional_fee_5(4, 0)` gives `0`.
- `fractional_fee_5(65535, 1)` gives `3276`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-140/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-140`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
