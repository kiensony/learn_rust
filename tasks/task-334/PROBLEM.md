# task-334 — Copy a trail coordinate

Return two identical copies of the coordinate. Preserve both signed components, using the Copy property of the tuple.

```rust
pub fn both_points(point: (i16, i16)) -> [(i16, i16); 2]
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `both_points((1, 3))` gives `[(1, 3), (1, 3)]`.
- `both_points((-4, 2))` gives `[(-4, 2), (-4, 2)]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-334/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-334`.

[Module reading](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
