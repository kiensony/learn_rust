# task-1583 — Feature lanes · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Each byte counts units. Observe the byte value multiplied by the active unit width. The default width is one. With the wide feature it is 16. Add a cfg-gated regression for wide and run cargo test --features wide from this package. Test both configurations. Put your own feature-specific case in a new integration test.

Return Some((minimum, maximum)) or None for an empty trace. Use Cargo features and cfg!. During review, point to the operation that enforces its contract.

```rust
pub fn feature_lanes_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `feature_lanes_bounds(&[])` gives `None` by default and `None` with `wide`.
- `feature_lanes_bounds(&[4])` gives `Some((4u32, 4u32))` by default and `Some((64u32, 64u32))` with `wide`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1583/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1583`.

[Module reading](https://doc.rust-lang.org/cargo/reference/features.html)
