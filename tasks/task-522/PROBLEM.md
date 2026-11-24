# task-522 — Measure Unicode text · marks removed at both ends (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. After selecting the field, count its Unicode scalar values. An emoji occupies one scalar value in the examples.

```rust
pub fn marks_removed_at_both_ends_slash(text: &str) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `marks_removed_at_both_ends_slash("path/1/12")` gives `4`.
- `marks_removed_at_both_ends_slash("path/123")` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-522/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-522`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
