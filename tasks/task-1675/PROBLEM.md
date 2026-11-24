# task-1675 — Patch version · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Read whitespace-separated release tags in a restricted major.minor.patch format, exactly three decimal components in 0..=255, leading zeroes permitted. Ignore invalid tags. Observe the patch component for every valid tag. During review identify one downstream change that would require a major release of this public API.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use a release-tag parser that rejects unsupported syntax. During review, point to the operation that enforces its contract.

```rust
pub fn patch_version_changes(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `patch_version_changes("")` gives `vec![]`.
- `patch_version_changes("0.0.1")` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1675/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1675`.

[Module reading](https://doc.rust-lang.org/cargo/reference/semver.html)
