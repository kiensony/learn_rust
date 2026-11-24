# task-382 — Inspect a borrowed label · marks removed at both ends (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept a shared reference and return the byte count of the selected field. The caller keeps its String.

```rust
pub fn marks_removed_at_both_ends_pipe(text: &String) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `marks_removed_at_both_ends_pipe(&String::from("first\n123"))` gives `3`.
- `marks_removed_at_both_ends_pipe(&String::from("header\n猫|123"))` gives `7`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-382/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-382`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
