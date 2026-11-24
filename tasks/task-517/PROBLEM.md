# task-517 — Measure Unicode text · shortest field with first tie (:)

Treat ':' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. After selecting the field, count its Unicode scalar values. An emoji occupies one scalar value in the examples.

```rust
pub fn shortest_field_with_first_tie_colon(text: &str) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shortest_field_with_first_tie_colon("hi:kit")` gives `2`.
- `shortest_field_with_first_tie_colon(":é:a")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-517/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-517`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
