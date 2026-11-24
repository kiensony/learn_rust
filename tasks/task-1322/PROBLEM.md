# task-1322 — Flag fields · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the odd parsed values.

Return the sum of all observations, widening each one before adding. Use str::parse and explicit handling of parse errors. During review, point to the operation that enforces its contract.

```rust
pub fn flag_fields_total(input: &str) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `flag_fields_total("")` gives `0`.
- `flag_fields_total("1")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1322/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1322`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)
