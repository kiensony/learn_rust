# task-541 — Measure Unicode text · closing marks removed (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. After selecting the field, count its Unicode scalar values. An emoji occupies one scalar value in the examples.

```rust
pub fn closing_marks_removed_equals(text: &str) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `closing_marks_removed_equals("72;12")` gives `2`.
- `closing_marks_removed_equals("猫=12;note")` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-541/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-541`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
