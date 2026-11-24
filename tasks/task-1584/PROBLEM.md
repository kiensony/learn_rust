# task-1584 — Feature lanes · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Each byte counts units. Observe the byte value multiplied by the active unit width. The default width is one. With the wide feature it is 16. Add a cfg-gated regression for wide and run cargo test --features wide from this package. Test both configurations. Put your own feature-specific case in a new integration test.

Keep the first observation and every later one strictly above all earlier observations. Use Cargo features and cfg!. During review, point to the operation that enforces its contract.

```rust
pub fn feature_lanes_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `feature_lanes_peaks(&[])` gives `vec![]` by default and `vec![]` with `wide`.
- `feature_lanes_peaks(&[4])` gives `vec![4u32]` by default and `vec![64u32]` with `wide`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1584/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1584`.

[Module reading](https://doc.rust-lang.org/cargo/reference/features.html)
