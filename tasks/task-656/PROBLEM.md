# task-656 — Route an event · leading field (:)

Treat ':' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. For Data return Some(selected byte length), for Heartbeat return Some(0), and for End return None. Match all three variants explicitly.

```rust
pub fn leading_field_colon(event: Event<'_>) -> Option<usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_colon(Event::Data("hi:kit"))` gives `Some(2)`.
- `leading_field_colon(Event::Data(":é:a"))` gives `Some(0)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-656/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-656`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
