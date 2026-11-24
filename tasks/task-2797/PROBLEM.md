# task-2797 — Parsed key lengths · run ledger

Design a reusable settings parser. Separate fields by semicolons, with exact key=value spelling. Valid keys are nonempty ASCII letters. Values are decimal ASCII bytes in 0..=255. Leading zeroes are allowed. Skip invalid fields, preserve input order unless the report asks otherwise. Observe the byte length of each valid key.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use borrowed parsing followed by an independently testable projection. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn parsed_key_lengths_run_ledger(input: &str) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `parsed_key_lengths_run_ledger("xx=3;z=0;LONG=12")` gives `vec![(2u32, 1usize), (1u32, 1usize), (4u32, 1usize)]`.
- `parsed_key_lengths_run_ledger("")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2797/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2797`.

[Module reading](https://doc.rust-lang.org/std/str/)
