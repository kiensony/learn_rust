# task-095 — Mirror a coordinate · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Find the distance between the coordinate value and the marker 8.

```rust
pub fn mirror_a_coordinate_8(start: u16, end: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `mirror_a_coordinate_8(4, 0)` gives `4`.
- `mirror_a_coordinate_8(0, 2)` gives `6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-095/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-095`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
