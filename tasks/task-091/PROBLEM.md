# task-091 — Count packs · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Each pack contains 8 pencils. Return the number of pencils in value packs.

```rust
pub fn count_packs_8(start: u16, end: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `count_packs_8(4, 0)` gives `32`.
- `count_packs_8(13, 1)` gives `96`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-091/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-091`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
