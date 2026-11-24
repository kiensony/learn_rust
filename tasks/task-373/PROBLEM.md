# task-373 — Inspect a borrowed label · remainder after first mark (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept a shared reference and return the byte count of the selected field. The caller keeps its String.

```rust
pub fn remainder_after_first_mark_slash(text: &String) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_slash(&String::from("path/1/12"))` gives `2`.
- `remainder_after_first_mark_slash(&String::from("path/123"))` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-373/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-373`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
