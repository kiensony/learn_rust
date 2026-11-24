# task-345 — Consume a label · first nonempty field (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Take ownership of the input and return the selected text as an owned String. The caller must be able to move the result into a new binding.

```rust
pub fn first_nonempty_field_equals(mut text: String) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `first_nonempty_field_equals(String::from("72;12"))` gives `"72"`.
- `first_nonempty_field_equals(String::from("猫=12;note"))` gives `"猫"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-345/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-345`.

[Module reading](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
