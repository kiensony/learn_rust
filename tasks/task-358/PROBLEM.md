# task-358 — Inspect a borrowed label · leading field (:)

Treat ':' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept a shared reference and return the byte count of the selected field. The caller keeps its String.

```rust
pub fn leading_field_colon(text: &String) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_colon(&String::from("hi:kit"))` gives `2`.
- `leading_field_colon(&String::from(":é:a"))` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-358/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-358`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
