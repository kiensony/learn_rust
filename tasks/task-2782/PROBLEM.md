# task-2782 — Parsed defaults · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Design a reusable settings parser. Separate fields by semicolons, with exact key=value spelling. Valid keys are nonempty ASCII letters. Values are decimal ASCII bytes in 0..=255. Leading zeroes are allowed. Skip invalid fields, preserve input order unless the report asks otherwise. Observe the byte lengths of keys with value zero.

Return the sum of all observations, widening each one before adding. Use borrowed parsing followed by an independently testable projection. During review, point to the operation that enforces its contract.

```rust
pub fn parsed_defaults_total(input: &str) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `parsed_defaults_total("")` gives `0`.
- `parsed_defaults_total("a=1")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2782/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2782`.

[Module reading](https://doc.rust-lang.org/std/str/)
