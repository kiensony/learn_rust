# task-124 — Round up · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Return the smallest multiple of 3 at least as large as value.

```rust
pub fn round_up_3(first: u16, second: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `round_up_3(4, 0)` gives `6`.
- `round_up_3(13, 3)` gives `18`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-124/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-124`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
