# task-508 — Measure Unicode text · leading field (:)

Treat ':' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. After selecting the field, count its Unicode scalar values. An emoji occupies one scalar value in the examples.

```rust
pub fn leading_field_colon(text: &str) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_colon("hi:kit")` gives `2`.
- `leading_field_colon(":é:a")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-508/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-508`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
