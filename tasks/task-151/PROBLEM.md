# task-151 — Whole cartons · limited supply

A request is limited by available stock. Use the smaller count as value. Return the fewest cartons of size 10 that can hold value pieces.

```rust
pub fn whole_cartons_10(requested: u16, available: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `whole_cartons_10(65535, 4)` gives `1`.
- `whole_cartons_10(13, 14)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-151/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-151`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
