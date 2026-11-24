# task-635 — Read a record · shortest field with first tie (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn shortest_field_with_first_tie_pipe(record: &Record) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `shortest_field_with_first_tie_pipe(&Record { label: String::from("first\n123"), revision: 0 })` gives `3`.
- `shortest_field_with_first_tie_pipe(&Record { label: String::from("header\n猫|123"), revision: 1 })` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-635/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-635`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
