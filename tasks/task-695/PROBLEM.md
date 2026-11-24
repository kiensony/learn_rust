# task-695 — Route an event · leading field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. For Data return Some(selected byte length), for Heartbeat return Some(0), and for End return None. Match all three variants explicitly.

```rust
pub fn leading_field_hash(event: Event<'_>) -> Option<usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_hash(Event::Data("outside[red#blue]ignored"))` gives `Some(3)`.
- `leading_field_hash(Event::Data("unclosed[cat"))` gives `Some(0)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-695/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-695`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
