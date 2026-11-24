# task-677 — Route an event · leading marks removed (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. For Data return Some(selected byte length), for Heartbeat return Some(0), and for End return None. Match all three variants explicitly.

```rust
pub fn leading_marks_removed_pipe(event: Event<'_>) -> Option<usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_marks_removed_pipe(Event::Data("first\n123"))` gives `Some(3)`.
- `leading_marks_removed_pipe(Event::Data("header\n猫|123"))` gives `Some(7)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-677/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-677`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
