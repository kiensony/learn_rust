# task-537 — Measure Unicode text · shortest field with first tie (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. After selecting the field, count its Unicode scalar values. An emoji occupies one scalar value in the examples.

```rust
pub fn shortest_field_with_first_tie_pipe(text: &str) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `shortest_field_with_first_tie_pipe("first\n123")` gives `3`.
- `shortest_field_with_first_tie_pipe("header\n猫|123")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-537/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-537`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
