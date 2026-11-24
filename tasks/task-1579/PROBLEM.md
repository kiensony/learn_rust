# task-1579 — Feature packing · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Each byte counts units. Observe the byte value multiplied by the active unit width. The default width is one. With the wide feature it is 8. Add a cfg-gated regression for wide and run cargo test --features wide from this package. Test both configurations. Put your own feature-specific case in a new integration test.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use Cargo features and cfg!. During review, point to the operation that enforces its contract.

```rust
pub fn feature_packing_longest_run(input: &[u8]) -> u64
```

## Public examples

- `feature_packing_longest_run(&[])` gives `0` by default and `0` with `wide`.
- `feature_packing_longest_run(&[4])` gives `1` by default and `1` with `wide`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1579/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1579`.

[Module reading](https://doc.rust-lang.org/cargo/reference/features.html)
