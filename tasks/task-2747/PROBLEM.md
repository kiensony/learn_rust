# task-2747 — CLI extensions · run ledger

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the byte length of each file extension, omitting paths without an extension. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use std::path::Path component inspection. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn cli_extensions_run_ledger(input: &str) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `cli_extensions_run_ledger("input.release output.log /tmp/ok")` gives `vec![(7u32, 1usize), (3u32, 1usize)]`.
- `cli_extensions_run_ledger("")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2747/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2747`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
