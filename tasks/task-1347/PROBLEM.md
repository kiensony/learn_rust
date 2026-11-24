# task-1347 — Nibble fields · run ledger

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the high four bits of each parsed byte.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use str::parse and explicit handling of parse errors. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn nibble_fields_run_ledger(input: &str) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `nibble_fields_run_ledger("4,9,2,2,0")` gives `vec![(0u32, 5usize)]`.
- `nibble_fields_run_ledger("")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1347/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1347`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)
