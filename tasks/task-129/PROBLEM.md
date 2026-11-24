# task-129 — Nearest group · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Round value to the nearest multiple of 3. At the midpoint choose the higher multiple.

```rust
pub fn nearest_group_3(first: u16, second: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `nearest_group_3(4, 0)` gives `3`.
- `nearest_group_3(13, 3)` gives `15`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-129/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-129`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
