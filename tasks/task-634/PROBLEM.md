# task-634 — Read a record · longest field with last tie (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn longest_field_with_last_tie_pipe(record: &Record) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `longest_field_with_last_tie_pipe(&Record { label: String::from("first\n123"), revision: 0 })` gives `3`.
- `longest_field_with_last_tie_pipe(&Record { label: String::from("header\n猫|123"), revision: 1 })` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-634/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-634`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
