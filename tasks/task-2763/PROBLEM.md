# task-2763 — Parsed key lengths · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Design a reusable settings parser. Separate fields by semicolons, with exact key=value spelling. Valid keys are nonempty ASCII letters. Values are decimal ASCII bytes in 0..=255. Leading zeroes are allowed. Skip invalid fields, preserve input order unless the report asks otherwise. Observe the byte length of each valid key.

Return Some((minimum, maximum)) or None for an empty trace. Use borrowed parsing followed by an independently testable projection. During review, point to the operation that enforces its contract.

```rust
pub fn parsed_key_lengths_bounds(input: &str) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `parsed_key_lengths_bounds("")` gives `None`.
- `parsed_key_lengths_bounds("a=1")` gives `Some((1u32, 1u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2763/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2763`.

[Module reading](https://doc.rust-lang.org/std/str/)
