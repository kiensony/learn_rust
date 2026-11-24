# task-341 — Consume a label · closing marks removed (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Take ownership of the input and return the selected text as an owned String. The caller must be able to move the result into a new binding.

```rust
pub fn closing_marks_removed_equals(mut text: String) -> String
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `closing_marks_removed_equals(String::from("72;12"))` gives `"72"`.
- `closing_marks_removed_equals(String::from("猫=12;note"))` gives `"猫=12"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-341/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-341`.

[Module reading](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
