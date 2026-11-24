# task-538 — Measure Unicode text · leading field (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. After selecting the field, count its Unicode scalar values. An emoji occupies one scalar value in the examples.

```rust
pub fn leading_field_equals(text: &str) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_equals("72;12")` gives `2`.
- `leading_field_equals("猫=12;note")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-538/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-538`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
