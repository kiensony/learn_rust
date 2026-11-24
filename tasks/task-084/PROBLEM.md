# task-084 — Loose tiles · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. After making rows of 5, return the number of tiles left over.

```rust
pub fn loose_tiles_5(issued: u16, returned: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `loose_tiles_5(4, 0)` gives `4`.
- `loose_tiles_5(13, 1)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-084/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-084`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
