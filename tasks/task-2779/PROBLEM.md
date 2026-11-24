# task-2779 — Repeated keys · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Design a reusable settings parser. Separate fields by semicolons, with exact key=value spelling. Valid keys are nonempty ASCII letters. Values are decimal ASCII bytes in 0..=255. Leading zeroes are allowed. Skip invalid fields, preserve input order unless the report asks otherwise. Observe values from the second and later occurrence of every case-sensitive key in input order.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use borrowed parsing followed by an independently testable projection. During review, point to the operation that enforces its contract.

```rust
pub fn repeated_keys_longest_run(input: &str) -> u64
```

## Public examples

- `repeated_keys_longest_run("")` gives `0`.
- `repeated_keys_longest_run("a=1")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2779/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2779`.

[Module reading](https://doc.rust-lang.org/std/str/)
