# task-1687 — Preview version · distinct

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Read whitespace-separated release tags in a restricted major.minor.patch format, exactly three decimal components in 0..=255, leading zeroes permitted. Ignore invalid tags. Observe the one for major zero and zero otherwise for every valid tag. During review identify one downstream change that would require a major release of this public API.

Return distinct observations in ascending order. Use a release-tag parser that rejects unsupported syntax. During review, point to the operation that enforces its contract.

```rust
pub fn preview_version_distinct(input: &str) -> Vec<u32>
```

## Public examples

- `preview_version_distinct("")` gives `vec![]`.
- `preview_version_distinct("0.0.1")` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1687/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1687`.

[Module reading](https://doc.rust-lang.org/cargo/reference/semver.html)
