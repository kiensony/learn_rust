# task-289 — Wrap a message · plus signs

Return text surrounded on both sides by '+'.

```rust
pub fn wrap_a_message_plus(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `wrap_a_message_plus("cat")` gives `"+cat+"`.
- `wrap_a_message_plus("+hi+")` gives `"++hi++"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-289/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-289`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
