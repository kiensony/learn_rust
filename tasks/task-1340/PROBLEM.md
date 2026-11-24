# task-1340 — Field indexes · reverse

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the zero-based indexes of parsed zero values, counting only valid fields.

Return the observations in reverse order. Retain duplicate observations. Use str::parse and explicit handling of parse errors. During review, point to the operation that enforces its contract.

```rust
pub fn field_indexes_reverse(input: &str) -> Vec<u32>
```

## Public examples

- `field_indexes_reverse("")` gives `vec![]`.
- `field_indexes_reverse("1")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1340/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1340`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)
