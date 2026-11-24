# task-2773 — Repeated keys · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Design a reusable settings parser. Separate fields by semicolons, with exact key=value spelling. Valid keys are nonempty ASCII letters. Values are decimal ASCII bytes in 0..=255. Leading zeroes are allowed. Skip invalid fields, preserve input order unless the report asks otherwise. Observe values from the second and later occurrence of every case-sensitive key in input order.

Return Some((minimum, maximum)) or None for an empty trace. Use borrowed parsing followed by an independently testable projection. During review, point to the operation that enforces its contract.

```rust
pub fn repeated_keys_bounds(input: &str) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `repeated_keys_bounds("")` gives `None`.
- `repeated_keys_bounds("a=1")` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2773/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2773`.

[Module reading](https://doc.rust-lang.org/std/str/)
