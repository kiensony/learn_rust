# task-566 — Return a borrowed field · shortest field with first tie (:)

Treat ':' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller.

```rust
pub fn shortest_field_with_first_tie_colon<'a>(text: &'a str) -> &'a str
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shortest_field_with_first_tie_colon("hi:kit")` gives `"hi"`.
- `shortest_field_with_first_tie_colon(":é:a")` gives `""`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-566/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-566`.

[Module reading](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
