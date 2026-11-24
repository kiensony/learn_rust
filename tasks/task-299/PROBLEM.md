# task-299 — Wrap a message · exclamation marks

Return text surrounded on both sides by '!'.

```rust
pub fn wrap_a_message_bang(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `wrap_a_message_bang("cat")` gives `"!cat!"`.
- `wrap_a_message_bang("!hi!")` gives `"!!hi!!"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-299/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-299`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
