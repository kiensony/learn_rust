# task-2783 — Parsed defaults · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Design a reusable settings parser. Separate fields by semicolons, with exact key=value spelling. Valid keys are nonempty ASCII letters. Values are decimal ASCII bytes in 0..=255. Leading zeroes are allowed. Skip invalid fields, preserve input order unless the report asks otherwise. Observe the byte lengths of keys with value zero.

Return Some((minimum, maximum)) or None for an empty trace. Use borrowed parsing followed by an independently testable projection. During review, point to the operation that enforces its contract.

```rust
pub fn parsed_defaults_bounds(input: &str) -> Option<(u32, u32)>
```

## Public examples

- `parsed_defaults_bounds("")` gives `None`.
- `parsed_defaults_bounds("a=1")` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2783/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2783`.

[Module reading](https://doc.rust-lang.org/std/str/)
