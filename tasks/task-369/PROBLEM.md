# task-369 — Inspect a borrowed label · closing field (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept a shared reference and return the byte count of the selected field. The caller keeps its String.

```rust
pub fn closing_field_slash(text: &String) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `closing_field_slash(&String::from("path/1/12"))` gives `2`.
- `closing_field_slash(&String::from("path/123"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-369/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-369`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
