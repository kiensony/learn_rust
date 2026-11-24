# task-548 — Measure Unicode text · leading field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. After selecting the field, count its Unicode scalar values. An emoji occupies one scalar value in the examples.

```rust
pub fn leading_field_hash(text: &str) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_hash("outside[red#blue]ignored")` gives `3`.
- `leading_field_hash("[猫###a]")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-548/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-548`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
