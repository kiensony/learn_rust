# task-666 — Route an event · closing field (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. For Data return Some(selected byte length), for Heartbeat return Some(0), and for End return None. Match all three variants explicitly.

```rust
pub fn closing_field_slash(event: Event<'_>) -> Option<usize>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `closing_field_slash(Event::Data("path/1/12"))` gives `Some(2)`.
- `closing_field_slash(Event::Data("path/123"))` gives `Some(3)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-666/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-666`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
