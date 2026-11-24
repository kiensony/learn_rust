# task-073 — Complete rows · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Arrange value tiles in rows of 3. Return the number of complete rows.

```rust
pub fn complete_rows_3(first: u16, second: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `complete_rows_3(4, 0)` gives `1`.
- `complete_rows_3(13, 3)` gives `5`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-073/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-073`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
