# task-682 — Route an event · first nonempty field (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. For Data return Some(selected byte length), for Heartbeat return Some(0), and for End return None. Match all three variants explicitly.

```rust
pub fn first_nonempty_field_pipe(event: Event<'_>) -> Option<usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `first_nonempty_field_pipe(Event::Data("first\n123"))` gives `Some(3)`.
- `first_nonempty_field_pipe(Event::Data("hi|kit"))` gives `Some(2)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-682/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-682`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
