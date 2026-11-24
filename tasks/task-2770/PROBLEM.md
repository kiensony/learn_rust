# task-2770 — Parsed key lengths · reverse

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Design a reusable settings parser. Separate fields by semicolons, with exact key=value spelling. Valid keys are nonempty ASCII letters. Values are decimal ASCII bytes in 0..=255. Leading zeroes are allowed. Skip invalid fields, preserve input order unless the report asks otherwise. Observe the byte length of each valid key.

Return the observations in reverse order. Retain duplicate observations. Use borrowed parsing followed by an independently testable projection. During review, point to the operation that enforces its contract.

```rust
pub fn parsed_key_lengths_reverse(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `parsed_key_lengths_reverse("")` gives `vec![]`.
- `parsed_key_lengths_reverse("a=1")` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2770/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2770`.

[Module reading](https://doc.rust-lang.org/std/str/)
