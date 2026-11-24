# task-1297 — Positional options · run ledger

Build a child command named report. The input is a list of UTF-8 tokens separated by ASCII whitespace, with no shell quoting. Inspect the byte lengths of arguments whose first character is not a hyphen. Do not spawn the child.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use Command::args and Command::get_args. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn positional_options_run_ledger(input: &str) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `positional_options_run_ledger("a --bad --level=x b")` gives `vec![(1u32, 2usize)]`.
- `positional_options_run_ledger("")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1297/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1297`.

[Module reading](https://doc.rust-lang.org/std/process/)
