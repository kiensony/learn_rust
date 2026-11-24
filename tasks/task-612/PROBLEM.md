# task-612 — Read a record · prefix before final mark (:)

Treat ':' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn prefix_before_final_mark_colon(record: &Record) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `prefix_before_final_mark_colon(&Record { label: String::from("hi:kit"), revision: 0 })` gives `2`.
- `prefix_before_final_mark_colon(&Record { label: String::from(":é:a"), revision: 1 })` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-612/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-612`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
