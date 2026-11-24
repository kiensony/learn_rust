# task-317 — Consume a label · shortest field with first tie (:)

Treat ':' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Take ownership of the input and return the selected text as an owned String. The caller must be able to move the result into a new binding.

```rust
pub fn shortest_field_with_first_tie_colon(mut text: String) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shortest_field_with_first_tie_colon(String::from("hi:kit"))` gives `"hi"`.
- `shortest_field_with_first_tie_colon(String::from(":é:a"))` gives `""`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-317/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-317`.

[Module reading](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
