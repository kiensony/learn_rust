# task-104 — Loose tiles · limited supply

A request is limited by available stock. Use the smaller count as value. After making rows of 10, return the number of tiles left over.

```rust
pub fn loose_tiles_10(requested: u16, available: u16) -> u32
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `loose_tiles_10(65535, 4)` gives `4`.
- `loose_tiles_10(13, 14)` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-104/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-104`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
