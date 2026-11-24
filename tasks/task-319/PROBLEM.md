# task-319 — Consume a label · closing field (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Take ownership of the input and return the selected text as an owned String. The caller must be able to move the result into a new binding.

```rust
pub fn closing_field_slash(mut text: String) -> String
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `closing_field_slash(String::from("path/1/12"))` gives `"12"`.
- `closing_field_slash(String::from("path/123"))` gives `"123"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-319/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-319`.

[Module reading](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
