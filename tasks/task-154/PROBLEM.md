# task-154 — Round up · limited supply

A request is limited by available stock. Use the smaller count as value. Return the smallest multiple of 10 at least as large as value.

```rust
pub fn round_up_10(requested: u16, available: u16) -> u32
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `round_up_10(65535, 4)` gives `10`.
- `round_up_10(13, 14)` gives `20`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-154/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-154`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
