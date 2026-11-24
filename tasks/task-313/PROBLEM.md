# task-313 — Consume a label · marks removed at both ends (:)

Treat ':' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Take ownership of the input and return the selected text as an owned String. The caller must be able to move the result into a new binding.

```rust
pub fn marks_removed_at_both_ends_colon(mut text: String) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `marks_removed_at_both_ends_colon(String::from("hi:kit"))` gives `"hi:kit"`.
- `marks_removed_at_both_ends_colon(String::from(":é:a"))` gives `"é:a"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-313/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-313`.

[Module reading](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
