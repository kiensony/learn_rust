# task-2765 — Parsed key lengths · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Design a reusable settings parser. Separate fields by semicolons, with exact key=value spelling. Valid keys are nonempty ASCII letters. Values are decimal ASCII bytes in 0..=255. Leading zeroes are allowed. Skip invalid fields, preserve input order unless the report asks otherwise. Observe the byte length of each valid key.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use borrowed parsing followed by an independently testable projection. During review, point to the operation that enforces its contract.

```rust
pub fn parsed_key_lengths_changes(input: &str) -> Vec<u32>
```

## Public examples

- `parsed_key_lengths_changes("")` gives `vec![]`.
- `parsed_key_lengths_changes("a=1")` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2765/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2765`.

[Module reading](https://doc.rust-lang.org/std/str/)
