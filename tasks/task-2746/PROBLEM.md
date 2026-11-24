# task-2746 — CLI basenames · threshold crossing

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the byte length of each path's final normal component. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

Return the zero-based position and value of the first observation at least limit. Return None if no observation qualifies. The position refers to the derived sequence.

Use std::path::Path component inspection. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn cli_basenames_threshold_crossing(input: &str, limit: u32) -> Option<(usize, u32)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `cli_basenames_threshold_crossing("a a/z.rs a/b.rs .cargo/config", 0)` gives `Some((0, 1))`.
- `cli_basenames_threshold_crossing("", 0)` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2746/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2746`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
