# task-640 — Read a record · marks removed at both ends (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn marks_removed_at_both_ends_equals(record: &Record) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `marks_removed_at_both_ends_equals(&Record { label: String::from("72;12"), revision: 0 })` gives `2`.
- `marks_removed_at_both_ends_equals(&Record { label: String::from("猫=12;note"), revision: 1 })` gives `7`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-640/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-640`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
