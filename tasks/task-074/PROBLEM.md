# task-074 — Loose tiles · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. After making rows of 3, return the number of tiles left over.

```rust
pub fn loose_tiles_3(first: u16, second: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `loose_tiles_3(4, 0)` gives `1`.
- `loose_tiles_3(0, 6)` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-074/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-074`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
