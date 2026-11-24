# task-644 — Read a record · longest field with last tie (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn longest_field_with_last_tie_equals(record: &Record) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `longest_field_with_last_tie_equals(&Record { label: String::from("72;12"), revision: 0 })` gives `2`.
- `longest_field_with_last_tie_equals(&Record { label: String::from("猫=12;note"), revision: 1 })` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-644/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-644`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
