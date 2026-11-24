# task-1349 — Field indexes · frequent readings

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the zero-based indexes of parsed zero values, counting only valid fields.

Return up to keep distinct observations ordered by descending frequency, breaking ties by first appearance in the trace. Return an empty vector for keep zero. Each observation value appears at most once in the result.

Use str::parse and explicit handling of parse errors. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn field_indexes_frequent_readings(input: &str, keep: usize) -> Vec<u32>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `field_indexes_frequent_readings("0,0,0", 3)` gives `vec![0u32, 1u32, 2u32]`.
- `field_indexes_frequent_readings("", 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1349/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1349`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)
