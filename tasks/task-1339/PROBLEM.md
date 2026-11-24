# task-1339 — Field indexes · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the zero-based indexes of parsed zero values, counting only valid fields.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use str::parse and explicit handling of parse errors. During review, point to the operation that enforces its contract.

```rust
pub fn field_indexes_longest_run(input: &str) -> u64
```

## Public examples

- `field_indexes_longest_run("")` gives `0`.
- `field_indexes_longest_run("1")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1339/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1339`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)
