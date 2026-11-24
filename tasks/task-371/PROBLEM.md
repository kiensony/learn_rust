# task-371 — Inspect a borrowed label · closing marks removed (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept a shared reference and return the byte count of the selected field. The caller keeps its String.

```rust
pub fn closing_marks_removed_slash(text: &String) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_marks_removed_slash(&String::from("path/1/12"))` gives `4`.
- `closing_marks_removed_slash(&String::from("path/123"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-371/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-371`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
