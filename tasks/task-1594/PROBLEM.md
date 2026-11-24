# task-1594 — Feature pages · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Each byte counts units. Observe the byte value multiplied by the active unit width. The default width is one. With the wide feature it is 32. Add a cfg-gated regression for wide and run cargo test --features wide from this package. Test both configurations. Put your own feature-specific case in a new integration test.

Keep the first observation and every later one strictly above all earlier observations. Use Cargo features and cfg!. During review, point to the operation that enforces its contract.

```rust
pub fn feature_pages_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `feature_pages_peaks(&[])` gives `vec![]` by default and `vec![]` with `wide`.
- `feature_pages_peaks(&[4])` gives `vec![4u32]` by default and `vec![128u32]` with `wide`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1594/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1594`.

[Module reading](https://doc.rust-lang.org/cargo/reference/features.html)
