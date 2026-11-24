# task-1700 — Release line · replay mismatch

Read whitespace-separated release tags in a restricted major.minor.patch format, exactly three decimal components in 0..=255, leading zeroes permitted. Ignore invalid tags. Observe the major component times 256 plus the minor component for every valid tag. During review identify one downstream change that would require a major release of this public API.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use a release-tag parser that rejects unsupported syntax. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn release_line_replay_mismatch(input: &str, replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `release_line_replay_mismatch("1.0.0 1.2.0 2.0.0", &[256, 258])` gives `Some(2)`.
- `release_line_replay_mismatch("", &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1700/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1700`.

[Module reading](https://doc.rust-lang.org/cargo/reference/semver.html)
