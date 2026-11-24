# task-659 — Route an event · closing marks removed (:)

Treat ':' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. For Data return Some(selected byte length), for Heartbeat return Some(0), and for End return None. Match all three variants explicitly.

```rust
pub fn closing_marks_removed_colon(event: Event<'_>) -> Option<usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_marks_removed_colon(Event::Data("hi:kit"))` gives `Some(6)`.
- `closing_marks_removed_colon(Event::Data(":é:a"))` gives `Some(5)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-659/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-659`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
