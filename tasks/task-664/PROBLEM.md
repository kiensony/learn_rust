# task-664 — Route an event · shortest field with first tie (:)

Treat ':' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. For Data return Some(selected byte length), for Heartbeat return Some(0), and for End return None. Match all three variants explicitly.

```rust
pub fn shortest_field_with_first_tie_colon(event: Event<'_>) -> Option<usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shortest_field_with_first_tie_colon(Event::Data("hi:kit"))` gives `Some(2)`.
- `shortest_field_with_first_tie_colon(Event::Data(":é:a"))` gives `Some(0)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-664/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-664`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
