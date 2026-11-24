# task-127 — Fit a byte · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Add 3 to value and return the result capped at 255.

```rust
pub fn fit_a_byte_3(first: u16, second: u16) -> u32
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `fit_a_byte_3(4, 0)` gives `7`.
- `fit_a_byte_3(13, 3)` gives `19`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-127/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-127`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
