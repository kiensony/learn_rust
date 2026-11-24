# task-2776 — Repeated keys · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Design a reusable settings parser. Separate fields by semicolons, with exact key=value spelling. Valid keys are nonempty ASCII letters. Values are decimal ASCII bytes in 0..=255. Leading zeroes are allowed. Skip invalid fields, preserve input order unless the report asks otherwise. Observe values from the second and later occurrence of every case-sensitive key in input order.

Count adjacent pairs for which the second observation is strictly smaller. Use borrowed parsing followed by an independently testable projection. During review, point to the operation that enforces its contract.

```rust
pub fn repeated_keys_descending_steps(input: &str) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `repeated_keys_descending_steps("")` gives `0`.
- `repeated_keys_descending_steps("a=1")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2776/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2776`.

[Module reading](https://doc.rust-lang.org/std/str/)
