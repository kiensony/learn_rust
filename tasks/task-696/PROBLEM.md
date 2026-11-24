# task-696 — Route an event · closing field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. For Data return Some(selected byte length), for Heartbeat return Some(0), and for End return None. Match all three variants explicitly.

```rust
pub fn closing_field_hash(event: Event<'_>) -> Option<usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_hash(Event::Data("outside[red#blue]ignored"))` gives `Some(4)`.
- `closing_field_hash(Event::Data("[猫###a]"))` gives `Some(1)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-696/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-696`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
