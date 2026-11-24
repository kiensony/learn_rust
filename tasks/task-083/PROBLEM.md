# task-083 — Complete rows · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Arrange value tiles in rows of 5. Return the number of complete rows.

```rust
pub fn complete_rows_5(issued: u16, returned: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `complete_rows_5(4, 0)` gives `0`.
- `complete_rows_5(13, 1)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-083/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-083`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
