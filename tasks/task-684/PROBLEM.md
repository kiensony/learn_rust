# task-684 — Route an event · shortest field with first tie (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. For Data return Some(selected byte length), for Heartbeat return Some(0), and for End return None. Match all three variants explicitly.

```rust
pub fn shortest_field_with_first_tie_pipe(event: Event<'_>) -> Option<usize>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `shortest_field_with_first_tie_pipe(Event::Data("first\n123"))` gives `Some(3)`.
- `shortest_field_with_first_tie_pipe(Event::Data("hi|kit"))` gives `Some(2)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-684/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-684`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
