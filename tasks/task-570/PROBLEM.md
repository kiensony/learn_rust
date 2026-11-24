# task-570 — Return a borrowed field · closing marks removed (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller.

```rust
pub fn closing_marks_removed_slash<'a>(text: &'a str) -> &'a str
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_marks_removed_slash("path/1/12")` gives `"1/12"`.
- `closing_marks_removed_slash("path/123")` gives `"123"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-570/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-570`.

[Module reading](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
