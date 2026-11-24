# task-562 — Return a borrowed field · marks removed at both ends (:)

Treat ':' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller.

```rust
pub fn marks_removed_at_both_ends_colon<'a>(text: &'a str) -> &'a str
```

Implement the public function in `src/lib.rs`.

## Public examples

- `marks_removed_at_both_ends_colon("hi:kit")` gives `"hi:kit"`.
- `marks_removed_at_both_ends_colon(":é:a")` gives `"é:a"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-562/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-562`.

[Module reading](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
