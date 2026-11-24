# task-1324 — Flag fields · peaks

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the odd parsed values.

Keep the first observation and every later one strictly above all earlier observations. Use str::parse and explicit handling of parse errors. During review, point to the operation that enforces its contract.

```rust
pub fn flag_fields_peaks(input: &str) -> Vec<u32>
```

## Public examples

- `flag_fields_peaks("")` gives `vec![]`.
- `flag_fields_peaks("1")` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1324/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1324`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)
