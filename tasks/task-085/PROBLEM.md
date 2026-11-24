# task-085 — Mirror a coordinate · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Find the distance between the coordinate value and the marker 5.

```rust
pub fn mirror_a_coordinate_5(issued: u16, returned: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `mirror_a_coordinate_5(4, 0)` gives `1`.
- `mirror_a_coordinate_5(13, 1)` gives `7`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-085/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-085`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
