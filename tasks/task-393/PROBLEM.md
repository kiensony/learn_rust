# task-393 — Inspect a borrowed label · remainder after first mark (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept a shared reference and return the byte count of the selected field. The caller keeps its String.

```rust
pub fn remainder_after_first_mark_equals(text: &String) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_equals(&String::from("72;12"))` gives `0`.
- `remainder_after_first_mark_equals(&String::from("猫=12;note"))` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-393/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-393`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
