# task-698 — Route an event · closing marks removed (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. For Data return Some(selected byte length), for Heartbeat return Some(0), and for End return None. Match all three variants explicitly.

```rust
pub fn closing_marks_removed_hash(event: Event<'_>) -> Option<usize>
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `closing_marks_removed_hash(Event::Data("outside[red#blue]ignored"))` gives `Some(8)`.
- `closing_marks_removed_hash(Event::Data("[猫###a]"))` gives `Some(7)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-698/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-698`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
