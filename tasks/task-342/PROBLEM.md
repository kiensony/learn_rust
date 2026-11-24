# task-342 — Consume a label · marks removed at both ends (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Take ownership of the input and return the selected text as an owned String. The caller must be able to move the result into a new binding.

```rust
pub fn marks_removed_at_both_ends_equals(mut text: String) -> String
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `marks_removed_at_both_ends_equals(String::from("72;12"))` gives `"72"`.
- `marks_removed_at_both_ends_equals(String::from("猫=12;note"))` gives `"猫=12"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-342/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-342`.

[Module reading](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
