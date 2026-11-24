# task-687 — Route an event · leading marks removed (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. For Data return Some(selected byte length), for Heartbeat return Some(0), and for End return None. Match all three variants explicitly.

```rust
pub fn leading_marks_removed_equals(event: Event<'_>) -> Option<usize>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `leading_marks_removed_equals(Event::Data("72;12"))` gives `Some(2)`.
- `leading_marks_removed_equals(Event::Data("猫=12;note"))` gives `Some(6)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-687/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-687`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
