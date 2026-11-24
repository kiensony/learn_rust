# task-574 — Return a borrowed field · first nonempty field (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller.

```rust
pub fn first_nonempty_field_slash<'a>(text: &'a str) -> &'a str
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `first_nonempty_field_slash("path/1/12")` gives `"1"`.
- `first_nonempty_field_slash("path/123")` gives `"123"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-574/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-574`.

[Module reading](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
