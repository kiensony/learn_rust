# task-1698 — Patch version · rolling load

Read whitespace-separated release tags in a restricted major.minor.patch format, exactly three decimal components in 0..=255, leading zeroes permitted. Ignore invalid tags. Observe the patch component for every valid tag. During review identify one downstream change that would require a major release of this public API.

For each complete sliding window of width observations, return its sum as u64. Consecutive windows overlap by width minus one observations. Width zero and widths greater than the trace length yield an empty vector.

Use a release-tag parser that rejects unsupported syntax. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn patch_version_rolling_load(input: &str, width: usize) -> Vec<u64>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `patch_version_rolling_load("12.4.8 2.9.3 12.4.8", 1)` gives `vec![8u64, 3u64, 8u64]`.
- `patch_version_rolling_load("", 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1698/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1698`.

[Module reading](https://doc.rust-lang.org/cargo/reference/semver.html)
