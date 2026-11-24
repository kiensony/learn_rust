# task-2793 — Final settings · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Design a reusable settings parser. Separate fields by semicolons, with exact key=value spelling. Valid keys are nonempty ASCII letters. Values are decimal ASCII bytes in 0..=255. Leading zeroes are allowed. Skip invalid fields, preserve input order unless the report asks otherwise. Observe the last value associated with each distinct key, in ascending ASCII key order.

Return Some((minimum, maximum)) or None for an empty trace. Use borrowed parsing followed by an independently testable projection. During review, point to the operation that enforces its contract.

```rust
pub fn final_settings_bounds(input: &str) -> Option<(u32, u32)>
```

## Public examples

- `final_settings_bounds("")` gives `None`.
- `final_settings_bounds("a=1")` gives `Some((1u32, 1u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2793/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2793`.

[Module reading](https://doc.rust-lang.org/std/str/)
