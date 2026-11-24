# task-1669 — Minor version · longest run

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Read whitespace-separated release tags in a restricted major.minor.patch format, exactly three decimal components in 0..=255, leading zeroes permitted. Ignore invalid tags. Observe the minor component for every valid tag. During review identify one downstream change that would require a major release of this public API.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use a release-tag parser that rejects unsupported syntax. During review, point to the operation that enforces its contract.

```rust
pub fn minor_version_longest_run(input: &str) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `minor_version_longest_run("")` gives `0`.
- `minor_version_longest_run("0.0.1")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1669/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1669`.

[Module reading](https://doc.rust-lang.org/cargo/reference/semver.html)
