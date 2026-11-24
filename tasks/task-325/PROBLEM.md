# task-325 — Consume a label · first nonempty field (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Take ownership of the input and return the selected text as an owned String. The caller must be able to move the result into a new binding.

```rust
pub fn first_nonempty_field_slash(mut text: String) -> String
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `first_nonempty_field_slash(String::from("path/1/12"))` gives `"1"`.
- `first_nonempty_field_slash(String::from("path/123"))` gives `"123"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-325/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-325`.

[Module reading](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
