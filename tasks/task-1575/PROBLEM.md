# task-1575 — Feature packing · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Each byte counts units. Observe the byte value multiplied by the active unit width. The default width is one. With the wide feature it is 8. Add a cfg-gated regression for wide and run cargo test --features wide from this package. Test both configurations. Put your own feature-specific case in a new integration test.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use Cargo features and cfg!. During review, point to the operation that enforces its contract.

```rust
pub fn feature_packing_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `feature_packing_changes(&[])` gives `vec![]` by default and `vec![]` with `wide`.
- `feature_packing_changes(&[4])` gives `vec![4u32]` by default and `vec![32u32]` with `wide`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1575/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1575`.

[Module reading](https://doc.rust-lang.org/cargo/reference/features.html)
