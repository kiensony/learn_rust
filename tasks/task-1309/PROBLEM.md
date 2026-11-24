# task-1309 — Decimal fields · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe each parsed byte.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use str::parse and explicit handling of parse errors. During review, point to the operation that enforces its contract.

```rust
pub fn decimal_fields_longest_run(input: &str) -> u64
```

## Public examples

- `decimal_fields_longest_run("")` gives `0`.
- `decimal_fields_longest_run("1")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1309/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1309`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)
