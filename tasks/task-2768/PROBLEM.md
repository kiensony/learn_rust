# task-2768 — Parsed key lengths · steps

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Design a reusable settings parser. Separate fields by semicolons, with exact key=value spelling. Valid keys are nonempty ASCII letters. Values are decimal ASCII bytes in 0..=255. Leading zeroes are allowed. Skip invalid fields, preserve input order unless the report asks otherwise. Observe the byte length of each valid key.

Return the absolute differences between each adjacent pair, in order. Use borrowed parsing followed by an independently testable projection. During review, point to the operation that enforces its contract.

```rust
pub fn parsed_key_lengths_steps(input: &str) -> Vec<u32>
```

## Public examples

- `parsed_key_lengths_steps("")` gives `vec![]`.
- `parsed_key_lengths_steps("a=1")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2768/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2768`.

[Module reading](https://doc.rust-lang.org/std/str/)
