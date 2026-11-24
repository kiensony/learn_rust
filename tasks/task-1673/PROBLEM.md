# task-1673 — Patch version · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Read whitespace-separated release tags in a restricted major.minor.patch format, exactly three decimal components in 0..=255, leading zeroes permitted. Ignore invalid tags. Observe the patch component for every valid tag. During review identify one downstream change that would require a major release of this public API.

Return Some((minimum, maximum)) or None for an empty trace. Use a release-tag parser that rejects unsupported syntax. During review, point to the operation that enforces its contract.

```rust
pub fn patch_version_bounds(input: &str) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `patch_version_bounds("")` gives `None`.
- `patch_version_bounds("0.0.1")` gives `Some((1u32, 1u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1673/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1673`.

[Module reading](https://doc.rust-lang.org/cargo/reference/semver.html)
