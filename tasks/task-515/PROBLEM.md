# task-515 — Measure Unicode text · first nonempty field (:)

Treat ':' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. After selecting the field, count its Unicode scalar values. An emoji occupies one scalar value in the examples.

```rust
pub fn first_nonempty_field_colon(text: &str) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `first_nonempty_field_colon("hi:kit")` gives `2`.
- `first_nonempty_field_colon(":é:a")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-515/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-515`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
