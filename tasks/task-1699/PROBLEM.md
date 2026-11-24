# task-1699 — Preview version · frequent readings

Read whitespace-separated release tags in a restricted major.minor.patch format, exactly three decimal components in 0..=255, leading zeroes permitted. Ignore invalid tags. Observe the one for major zero and zero otherwise for every valid tag. During review identify one downstream change that would require a major release of this public API.

Return up to keep distinct observations ordered by descending frequency, breaking ties by first appearance in the trace. Return an empty vector for keep zero. Each observation value appears at most once in the result.

Use a release-tag parser that rejects unsupported syntax. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn preview_version_frequent_readings(input: &str, keep: usize) -> Vec<u32>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `preview_version_frequent_readings("1.0.0 1.2.0 2.0.0", 3)` gives `vec![0u32]`.
- `preview_version_frequent_readings("", 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1699/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1699`.

[Module reading](https://doc.rust-lang.org/cargo/reference/semver.html)
