# task-1660 — Major version · reverse

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Read whitespace-separated release tags in a restricted major.minor.patch format, exactly three decimal components in 0..=255, leading zeroes permitted. Ignore invalid tags. Observe the major component for every valid tag. During review identify one downstream change that would require a major release of this public API.

Return the observations in reverse order. Retain duplicate observations. Use a release-tag parser that rejects unsupported syntax. During review, point to the operation that enforces its contract.

```rust
pub fn major_version_reverse(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `major_version_reverse("")` gives `vec![]`.
- `major_version_reverse("0.0.1")` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1660/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1660`.

[Module reading](https://doc.rust-lang.org/cargo/reference/semver.html)
