# task-1659 — Major version · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Read whitespace-separated release tags in a restricted major.minor.patch format, exactly three decimal components in 0..=255, leading zeroes permitted. Ignore invalid tags. Observe the major component for every valid tag. During review identify one downstream change that would require a major release of this public API.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use a release-tag parser that rejects unsupported syntax. During review, point to the operation that enforces its contract.

```rust
pub fn major_version_longest_run(input: &str) -> u64
```

## Public examples

- `major_version_longest_run("")` gives `0`.
- `major_version_longest_run("0.0.1")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1659/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1659`.

[Module reading](https://doc.rust-lang.org/cargo/reference/semver.html)
