# task-564 — Return a borrowed field · first nonempty field (:)

Treat ':' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller.

```rust
pub fn first_nonempty_field_colon<'a>(text: &'a str) -> &'a str
```

Implement the public function in `src/lib.rs`.

## Public examples

- `first_nonempty_field_colon("hi:kit")` gives `"hi"`.
- `first_nonempty_field_colon(":é:a")` gives `"é"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-564/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-564`.

[Module reading](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
