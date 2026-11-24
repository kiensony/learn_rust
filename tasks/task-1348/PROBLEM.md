# task-1348 — Flag fields · rolling load

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the odd parsed values.

For each complete sliding window of width observations, return its sum as u64. Consecutive windows overlap by width minus one observations. Width zero and widths greater than the trace length yield an empty vector.

Use str::parse and explicit handling of parse errors. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn flag_fields_rolling_load(input: &str, width: usize) -> Vec<u64>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `flag_fields_rolling_load("12,3,12,4,5", 1)` gives `vec![3u64, 5u64]`.
- `flag_fields_rolling_load("", 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1348/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1348`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)
