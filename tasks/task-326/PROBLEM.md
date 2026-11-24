# task-326 — Consume a label · longest field with last tie (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Take ownership of the input and return the selected text as an owned String. The caller must be able to move the result into a new binding.

```rust
pub fn longest_field_with_last_tie_slash(mut text: String) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `longest_field_with_last_tie_slash(String::from("path/1/12"))` gives `"12"`.
- `longest_field_with_last_tie_slash(String::from("path/123"))` gives `"123"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-326/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-326`.

[Module reading](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
