# task-075 — Mirror a coordinate · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Find the distance between the coordinate value and the marker 3.

```rust
pub fn mirror_a_coordinate_3(first: u16, second: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `mirror_a_coordinate_3(4, 0)` gives `1`.
- `mirror_a_coordinate_3(13, 3)` gives `13`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-075/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-075`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
