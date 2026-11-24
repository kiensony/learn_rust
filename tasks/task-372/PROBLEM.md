# task-372 — Inspect a borrowed label · marks removed at both ends (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept a shared reference and return the byte count of the selected field. The caller keeps its String.

```rust
pub fn marks_removed_at_both_ends_slash(text: &String) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `marks_removed_at_both_ends_slash(&String::from("path/1/12"))` gives `4`.
- `marks_removed_at_both_ends_slash(&String::from("path/123"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-372/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-372`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
