# task-1679 — Patch version · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Read whitespace-separated release tags in a restricted major.minor.patch format, exactly three decimal components in 0..=255, leading zeroes permitted. Ignore invalid tags. Observe the patch component for every valid tag. During review identify one downstream change that would require a major release of this public API.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use a release-tag parser that rejects unsupported syntax. During review, point to the operation that enforces its contract.

```rust
pub fn patch_version_longest_run(input: &str) -> u64
```

## Public examples

- `patch_version_longest_run("")` gives `0`.
- `patch_version_longest_run("0.0.1")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1679/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1679`.

[Module reading](https://doc.rust-lang.org/cargo/reference/semver.html)
