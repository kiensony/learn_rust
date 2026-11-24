# task-661 — Route an event · prefix before final mark (:)

Treat ':' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. For Data return Some(selected byte length), for Heartbeat return Some(0), and for End return None. Match all three variants explicitly.

```rust
pub fn prefix_before_final_mark_colon(event: Event<'_>) -> Option<usize>
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `prefix_before_final_mark_colon(Event::Data("hi:kit"))` gives `Some(2)`.
- `prefix_before_final_mark_colon(Event::Data(":é:a"))` gives `Some(3)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-661/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-661`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
