# task-300 — Remove a prefix · exclamation marks

Remove one leading '!' if present. Keep the rest exactly.

```rust
pub fn remove_a_prefix_bang(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remove_a_prefix_bang("cat")` gives `"cat"`.
- `remove_a_prefix_bang("!hi!")` gives `"hi!"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-300/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-300`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
