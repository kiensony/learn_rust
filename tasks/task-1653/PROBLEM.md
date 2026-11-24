# task-1653 — Major version · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Read whitespace-separated release tags in a restricted major.minor.patch format, exactly three decimal components in 0..=255, leading zeroes permitted. Ignore invalid tags. Observe the major component for every valid tag. During review identify one downstream change that would require a major release of this public API.

Return Some((minimum, maximum)) or None for an empty trace. Use a release-tag parser that rejects unsupported syntax. During review, point to the operation that enforces its contract.

```rust
pub fn major_version_bounds(input: &str) -> Option<(u32, u32)>
```

## Public examples

- `major_version_bounds("")` gives `None`.
- `major_version_bounds("0.0.1")` gives `Some((0u32, 0u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1653/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1653`.

[Module reading](https://doc.rust-lang.org/cargo/reference/semver.html)
