# task-2750 — CLI stems · replay mismatch

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the byte length of each file stem, preserving earlier extensions in compound names. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use std::path::Path component inspection. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn cli_stems_replay_mismatch(input: &str, replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `cli_stems_replay_mismatch("src/lib.rs readme docs/notes.md", &[3, 6])` gives `Some(2)`.
- `cli_stems_replay_mismatch("", &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2750/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2750`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
