# task-2796 — Parsed values · threshold crossing

Design a reusable settings parser. Separate fields by semicolons, with exact key=value spelling. Valid keys are nonempty ASCII letters. Values are decimal ASCII bytes in 0..=255. Leading zeroes are allowed. Skip invalid fields, preserve input order unless the report asks otherwise. Observe each parsed value.

Return the zero-based position and value of the first observation at least limit. Return None if no observation qualifies. The position refers to the derived sequence.

Use borrowed parsing followed by an independently testable projection. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn parsed_values_threshold_crossing(input: &str, limit: u32) -> Option<(usize, u32)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `parsed_values_threshold_crossing("a=0;b=1;b=2;z=255", 0)` gives `Some((0, 0))`.
- `parsed_values_threshold_crossing("", 0)` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2796/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2796`.

[Module reading](https://doc.rust-lang.org/std/str/)
