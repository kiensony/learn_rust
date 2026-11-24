# task-367 — Inspect a borrowed label · shortest field with first tie (:)

Treat ':' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept a shared reference and return the byte count of the selected field. The caller keeps its String.

```rust
pub fn shortest_field_with_first_tie_colon(text: &String) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shortest_field_with_first_tie_colon(&String::from("hi:kit"))` gives `2`.
- `shortest_field_with_first_tie_colon(&String::from(":é:a"))` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-367/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-367`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
