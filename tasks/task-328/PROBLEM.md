# task-328 — Consume a label · leading field (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Take ownership of the input and return the selected text as an owned String. The caller must be able to move the result into a new binding.

```rust
pub fn leading_field_pipe(mut text: String) -> String
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `leading_field_pipe(String::from("first\n123"))` gives `"123"`.
- `leading_field_pipe(String::from("header\n猫|123"))` gives `"猫"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-328/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-328`.

[Module reading](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
