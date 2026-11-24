# task-2775 — Repeated keys · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Design a reusable settings parser. Separate fields by semicolons, with exact key=value spelling. Valid keys are nonempty ASCII letters. Values are decimal ASCII bytes in 0..=255. Leading zeroes are allowed. Skip invalid fields, preserve input order unless the report asks otherwise. Observe values from the second and later occurrence of every case-sensitive key in input order.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use borrowed parsing followed by an independently testable projection. During review, point to the operation that enforces its contract.

```rust
pub fn repeated_keys_changes(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `repeated_keys_changes("")` gives `vec![]`.
- `repeated_keys_changes("a=1")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2775/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2775`.

[Module reading](https://doc.rust-lang.org/std/str/)
