# task-2784 — Parsed defaults · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Design a reusable settings parser. Separate fields by semicolons, with exact key=value spelling. Valid keys are nonempty ASCII letters. Values are decimal ASCII bytes in 0..=255. Leading zeroes are allowed. Skip invalid fields, preserve input order unless the report asks otherwise. Observe the byte lengths of keys with value zero.

Keep the first observation and every later one strictly above all earlier observations. Use borrowed parsing followed by an independently testable projection. During review, point to the operation that enforces its contract.

```rust
pub fn parsed_defaults_peaks(input: &str) -> Vec<u32>
```

## Public examples

- `parsed_defaults_peaks("")` gives `vec![]`.
- `parsed_defaults_peaks("a=1")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2784/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2784`.

[Module reading](https://doc.rust-lang.org/std/str/)
