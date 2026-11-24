# task-123 — Round down · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Return the greatest multiple of 3 no larger than value.

```rust
pub fn round_down_3(first: u16, second: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `round_down_3(4, 0)` gives `3`.
- `round_down_3(13, 3)` gives `15`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-123/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-123`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
