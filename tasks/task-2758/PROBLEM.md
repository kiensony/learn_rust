# task-2758 — Parsed values · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Design a reusable settings parser. Separate fields by semicolons, with exact key=value spelling. Valid keys are nonempty ASCII letters. Values are decimal ASCII bytes in 0..=255. Leading zeroes are allowed. Skip invalid fields, preserve input order unless the report asks otherwise. Observe each parsed value.

Return the absolute differences between each adjacent pair, in order. Use borrowed parsing followed by an independently testable projection. During review, point to the operation that enforces its contract.

```rust
pub fn parsed_values_steps(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `parsed_values_steps("")` gives `vec![]`.
- `parsed_values_steps("a=1")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2758/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2758`.

[Module reading](https://doc.rust-lang.org/std/str/)
