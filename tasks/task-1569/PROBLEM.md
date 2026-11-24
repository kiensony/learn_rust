# task-1569 — Feature padding · longest run

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Each byte counts units. Observe the byte value multiplied by the active unit width. The default width is one. With the wide feature it is 4. Add a cfg-gated regression for wide and run cargo test --features wide from this package. Test both configurations. Put your own feature-specific case in a new integration test.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use Cargo features and cfg!. During review, point to the operation that enforces its contract.

```rust
pub fn feature_padding_longest_run(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `feature_padding_longest_run(&[])` gives `0` by default and `0` with `wide`.
- `feature_padding_longest_run(&[4])` gives `1` by default and `1` with `wide`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1569/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1569`.

[Module reading](https://doc.rust-lang.org/cargo/reference/features.html)
