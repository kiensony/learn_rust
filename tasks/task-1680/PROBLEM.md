# task-1680 — Patch version · reverse

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Read whitespace-separated release tags in a restricted major.minor.patch format, exactly three decimal components in 0..=255, leading zeroes permitted. Ignore invalid tags. Observe the patch component for every valid tag. During review identify one downstream change that would require a major release of this public API.

Return the observations in reverse order. Retain duplicate observations. Use a release-tag parser that rejects unsupported syntax. During review, point to the operation that enforces its contract.

```rust
pub fn patch_version_reverse(input: &str) -> Vec<u32>
```

## Public examples

- `patch_version_reverse("")` gives `vec![]`.
- `patch_version_reverse("0.0.1")` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1680/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1680`.

[Module reading](https://doc.rust-lang.org/cargo/reference/semver.html)
