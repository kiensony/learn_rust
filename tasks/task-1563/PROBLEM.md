# task-1563 — Feature padding · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Each byte counts units. Observe the byte value multiplied by the active unit width. The default width is one. With the wide feature it is 4. Add a cfg-gated regression for wide and run cargo test --features wide from this package. Test both configurations. Put your own feature-specific case in a new integration test.

Return Some((minimum, maximum)) or None for an empty trace. Use Cargo features and cfg!. During review, point to the operation that enforces its contract.

```rust
pub fn feature_padding_bounds(input: &[u8]) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `feature_padding_bounds(&[])` gives `None` by default and `None` with `wide`.
- `feature_padding_bounds(&[4])` gives `Some((4u32, 4u32))` by default and `Some((16u32, 16u32))` with `wide`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1563/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1563`.

[Module reading](https://doc.rust-lang.org/cargo/reference/features.html)
