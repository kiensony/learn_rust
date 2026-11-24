# task-2749 — CLI Rust sources · frequent readings

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the zero-based argument index of every path whose extension is exactly rs. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

Return up to keep distinct observations ordered by descending frequency, breaking ties by first appearance in the trace. Return an empty vector for keep zero. Each observation value appears at most once in the result.

Use std::path::Path component inspection. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn cli_rust_sources_frequent_readings(input: &str, keep: usize) -> Vec<u32>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `cli_rust_sources_frequent_readings("a a/z.rs a/b.rs .cargo/config", 3)` gives `vec![1u32, 2u32]`.
- `cli_rust_sources_frequent_readings("", 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2749/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2749`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
