# task-584 — Return a borrowed field · first nonempty field (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller.

```rust
pub fn first_nonempty_field_pipe<'a>(text: &'a str) -> &'a str
```

Implement the public function in `src/lib.rs`.

## Public examples

- `first_nonempty_field_pipe("first\n123")` gives `"123"`.
- `first_nonempty_field_pipe("header\n猫|123")` gives `"猫"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-584/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-584`.

[Module reading](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
