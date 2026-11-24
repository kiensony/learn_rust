# task-138 — Digit place · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Read the base-5 digit in the second position from the right.

```rust
pub fn digit_place_5(issued: u16, returned: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `digit_place_5(4, 0)` gives `0`.
- `digit_place_5(13, 1)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-138/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-138`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
