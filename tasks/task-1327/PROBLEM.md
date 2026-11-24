# task-1327 — Flag fields · distinct

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the odd parsed values.

Return distinct observations in ascending order. Use str::parse and explicit handling of parse errors. During review, point to the operation that enforces its contract.

```rust
pub fn flag_fields_distinct(input: &str) -> Vec<u32>
```

## Public examples

- `flag_fields_distinct("")` gives `vec![]`.
- `flag_fields_distinct("1")` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1327/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1327`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)
