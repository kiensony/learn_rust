# task-668 — Route an event · closing marks removed (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. For Data return Some(selected byte length), for Heartbeat return Some(0), and for End return None. Match all three variants explicitly.

```rust
pub fn closing_marks_removed_slash(event: Event<'_>) -> Option<usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_marks_removed_slash(Event::Data("path/1/12"))` gives `Some(4)`.
- `closing_marks_removed_slash(Event::Data("path/123"))` gives `Some(3)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-668/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-668`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
