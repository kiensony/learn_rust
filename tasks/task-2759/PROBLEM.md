# task-2759 — Parsed values · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Design a reusable settings parser. Separate fields by semicolons, with exact key=value spelling. Valid keys are nonempty ASCII letters. Values are decimal ASCII bytes in 0..=255. Leading zeroes are allowed. Skip invalid fields, preserve input order unless the report asks otherwise. Observe each parsed value.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use borrowed parsing followed by an independently testable projection. During review, point to the operation that enforces its contract.

```rust
pub fn parsed_values_longest_run(input: &str) -> u64
```

## Public examples

- `parsed_values_longest_run("")` gives `0`.
- `parsed_values_longest_run("a=1")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2759/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2759`.

[Module reading](https://doc.rust-lang.org/std/str/)
