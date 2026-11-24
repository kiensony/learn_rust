# task-101 — Count packs · limited supply

A request is limited by available stock. Use the smaller count as value. Each pack contains 10 pencils. Return the number of pencils in value packs.

```rust
pub fn count_packs_10(requested: u16, available: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `count_packs_10(65535, 4)` gives `40`.
- `count_packs_10(13, 14)` gives `130`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-101/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-101`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
