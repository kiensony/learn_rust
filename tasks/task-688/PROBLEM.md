# task-688 — Route an event · closing marks removed (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. For Data return Some(selected byte length), for Heartbeat return Some(0), and for End return None. Match all three variants explicitly.

```rust
pub fn closing_marks_removed_equals(event: Event<'_>) -> Option<usize>
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `closing_marks_removed_equals(Event::Data("72;12"))` gives `Some(2)`.
- `closing_marks_removed_equals(Event::Data("猫=12;note"))` gives `Some(6)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-688/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-688`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
