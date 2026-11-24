# task-1693 — Release line · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Read whitespace-separated release tags in a restricted major.minor.patch format, exactly three decimal components in 0..=255, leading zeroes permitted. Ignore invalid tags. Observe the major component times 256 plus the minor component for every valid tag. During review identify one downstream change that would require a major release of this public API.

Return Some((minimum, maximum)) or None for an empty trace. Use a release-tag parser that rejects unsupported syntax. During review, point to the operation that enforces its contract.

```rust
pub fn release_line_bounds(input: &str) -> Option<(u32, u32)>
```

## Public examples

- `release_line_bounds("")` gives `None`.
- `release_line_bounds("0.0.1")` gives `Some((0u32, 0u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1693/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1693`.

[Module reading](https://doc.rust-lang.org/cargo/reference/semver.html)
