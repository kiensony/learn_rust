# task-379 — Inspect a borrowed label · closing field (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept a shared reference and return the byte count of the selected field. The caller keeps its String.

```rust
pub fn closing_field_pipe(text: &String) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_pipe(&String::from("first\n123"))` gives `3`.
- `closing_field_pipe(&String::from("|é|a"))` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-379/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-379`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
