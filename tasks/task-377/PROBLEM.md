# task-377 — Inspect a borrowed label · shortest field with first tie (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept a shared reference and return the byte count of the selected field. The caller keeps its String.

```rust
pub fn shortest_field_with_first_tie_slash(text: &String) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shortest_field_with_first_tie_slash(&String::from("path/1/12"))` gives `1`.
- `shortest_field_with_first_tie_slash(&String::from("path/123"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-377/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-377`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
