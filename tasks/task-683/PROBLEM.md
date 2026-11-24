# task-683 — Route an event · longest field with last tie (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. For Data return Some(selected byte length), for Heartbeat return Some(0), and for End return None. Match all three variants explicitly.

```rust
pub fn longest_field_with_last_tie_pipe(event: Event<'_>) -> Option<usize>
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `longest_field_with_last_tie_pipe(Event::Data("first\n123"))` gives `Some(3)`.
- `longest_field_with_last_tie_pipe(Event::Data("|é|a"))` gives `Some(2)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-683/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-683`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
