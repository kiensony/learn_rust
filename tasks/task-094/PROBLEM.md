# task-094 — Loose tiles · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. After making rows of 8, return the number of tiles left over.

```rust
pub fn loose_tiles_8(start: u16, end: u16) -> u32
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `loose_tiles_8(4, 0)` gives `4`.
- `loose_tiles_8(0, 2)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-094/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-094`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
