# task-337 — Consume a label · shortest field with first tie (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Take ownership of the input and return the selected text as an owned String. The caller must be able to move the result into a new binding.

```rust
pub fn shortest_field_with_first_tie_pipe(mut text: String) -> String
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `shortest_field_with_first_tie_pipe(String::from("first\n123"))` gives `"123"`.
- `shortest_field_with_first_tie_pipe(String::from("header\n猫|123"))` gives `"猫"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-337/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-337`.

[Module reading](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
