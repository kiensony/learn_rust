# task-938 — Call a supplied closure · remainder after first mark (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn remainder_after_first_mark_equals(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_equals("72;12", |s| s.chars().count() + 1)` gives `1`.
- `remainder_after_first_mark_equals("猫=12;note", |s| s.len() * 2)` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-938/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-938`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
