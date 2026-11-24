# task-2748 — CLI path depth · rolling load

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the count of normal path components in each path, excluding root, dot and dot-dot. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

For each complete sliding window of width observations, return its sum as u64. Consecutive windows overlap by width minus one observations. Width zero and widths greater than the trace length yield an empty vector.

Use std::path::Path component inspection. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn cli_path_depth_rolling_load(input: &str, width: usize) -> Vec<u64>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `cli_path_depth_rolling_load("a a/z.rs a/b.rs .cargo/config", 1)` gives `vec![1u64, 2u64, 2u64, 2u64]`.
- `cli_path_depth_rolling_load("", 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2748/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2748`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
