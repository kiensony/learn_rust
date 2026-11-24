# task-330 — Consume a label · leading marks removed (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Take ownership of the input and return the selected text as an owned String. The caller must be able to move the result into a new binding.

```rust
pub fn leading_marks_removed_pipe(mut text: String) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_marks_removed_pipe(String::from("first\n123"))` gives `"123"`.
- `leading_marks_removed_pipe(String::from("header\n猫|123"))` gives `"猫|123"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-330/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-330`.

[Module reading](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
