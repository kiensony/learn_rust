# task-1346 — Decimal fields · threshold crossing

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe each parsed byte.

Return the zero-based position and value of the first observation at least limit. Return None if no observation qualifies. The position refers to the derived sequence.

Use str::parse and explicit handling of parse errors. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn decimal_fields_threshold_crossing(input: &str, limit: u32) -> Option<(usize, u32)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `decimal_fields_threshold_crossing("4,9,2,2,0", 0)` gives `Some((0, 4))`.
- `decimal_fields_threshold_crossing("", 0)` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1346/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1346`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)
