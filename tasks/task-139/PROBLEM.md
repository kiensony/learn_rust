# task-139 — Nearest group · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Round value to the nearest multiple of 5. At the midpoint choose the higher multiple.

```rust
pub fn nearest_group_5(issued: u16, returned: u16) -> u32
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `nearest_group_5(4, 0)` gives `5`.
- `nearest_group_5(13, 1)` gives `10`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-139/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-139`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
