# task-381 — Inspect a borrowed label · closing marks removed (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept a shared reference and return the byte count of the selected field. The caller keeps its String.

```rust
pub fn closing_marks_removed_pipe(text: &String) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `closing_marks_removed_pipe(&String::from("first\n123"))` gives `3`.
- `closing_marks_removed_pipe(&String::from("header\n猫|123"))` gives `7`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-381/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-381`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
