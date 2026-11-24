# task-1662 — Minor version · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Read whitespace-separated release tags in a restricted major.minor.patch format, exactly three decimal components in 0..=255, leading zeroes permitted. Ignore invalid tags. Observe the minor component for every valid tag. During review identify one downstream change that would require a major release of this public API.

Return the sum of all observations, widening each one before adding. Use a release-tag parser that rejects unsupported syntax. During review, point to the operation that enforces its contract.

```rust
pub fn minor_version_total(input: &str) -> u64
```

## Public examples

- `minor_version_total("")` gives `0`.
- `minor_version_total("0.0.1")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1662/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1662`.

[Module reading](https://doc.rust-lang.org/cargo/reference/semver.html)
