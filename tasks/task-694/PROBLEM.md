# task-694 — Route an event · shortest field with first tie (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. For Data return Some(selected byte length), for Heartbeat return Some(0), and for End return None. Match all three variants explicitly.

```rust
pub fn shortest_field_with_first_tie_equals(event: Event<'_>) -> Option<usize>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `shortest_field_with_first_tie_equals(Event::Data("72;12"))` gives `Some(2)`.
- `shortest_field_with_first_tie_equals(Event::Data("=é=a"))` gives `Some(0)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-694/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-694`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
