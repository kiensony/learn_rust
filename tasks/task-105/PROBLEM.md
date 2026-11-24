# task-105 — Mirror a coordinate · limited supply

A request is limited by available stock. Use the smaller count as value. Find the distance between the coordinate value and the marker 10.

```rust
pub fn mirror_a_coordinate_10(requested: u16, available: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `mirror_a_coordinate_10(65535, 4)` gives `6`.
- `mirror_a_coordinate_10(13, 14)` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-105/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-105`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
