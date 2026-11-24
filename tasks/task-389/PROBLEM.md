# task-389 — Inspect a borrowed label · closing field (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept a shared reference and return the byte count of the selected field. The caller keeps its String.

```rust
pub fn closing_field_equals(text: &String) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_equals(&String::from("72;12"))` gives `2`.
- `closing_field_equals(&String::from("hi=kit"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-389/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-389`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
