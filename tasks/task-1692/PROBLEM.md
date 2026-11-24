# task-1692 — Release line · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Read whitespace-separated release tags in a restricted major.minor.patch format, exactly three decimal components in 0..=255, leading zeroes permitted. Ignore invalid tags. Observe the major component times 256 plus the minor component for every valid tag. During review identify one downstream change that would require a major release of this public API.

Return the sum of all observations, widening each one before adding. Use a release-tag parser that rejects unsupported syntax. During review, point to the operation that enforces its contract.

```rust
pub fn release_line_total(input: &str) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `release_line_total("")` gives `0`.
- `release_line_total("0.0.1")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1692/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1692`.

[Module reading](https://doc.rust-lang.org/cargo/reference/semver.html)
