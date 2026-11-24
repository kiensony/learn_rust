# task-572 — Return a borrowed field · remainder after first mark (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller.

```rust
pub fn remainder_after_first_mark_slash<'a>(text: &'a str) -> &'a str
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_slash("path/1/12")` gives `"12"`.
- `remainder_after_first_mark_slash("path/123")` gives `""`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-572/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-572`.

[Module reading](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
