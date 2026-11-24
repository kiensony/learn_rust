# task-673 — Route an event · longest field with last tie (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. For Data return Some(selected byte length), for Heartbeat return Some(0), and for End return None. Match all three variants explicitly.

```rust
pub fn longest_field_with_last_tie_slash(event: Event<'_>) -> Option<usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `longest_field_with_last_tie_slash(Event::Data("path/1/12"))` gives `Some(2)`.
- `longest_field_with_last_tie_slash(Event::Data("path/123"))` gives `Some(3)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-673/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-673`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
