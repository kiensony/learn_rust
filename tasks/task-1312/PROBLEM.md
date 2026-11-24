# task-1312 — Nibble fields · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the high four bits of each parsed byte.

Return the sum of all observations, widening each one before adding. Use str::parse and explicit handling of parse errors. During review, point to the operation that enforces its contract.

```rust
pub fn nibble_fields_total(input: &str) -> u64
```

## Public examples

- `nibble_fields_total("")` gives `0`.
- `nibble_fields_total("1")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1312/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1312`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)
