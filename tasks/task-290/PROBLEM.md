# task-290 — Remove a prefix · plus signs

Remove one leading '+' if present. Keep the rest exactly.

```rust
pub fn remove_a_prefix_plus(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remove_a_prefix_plus("cat")` gives `"cat"`.
- `remove_a_prefix_plus("+hi+")` gives `"hi+"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-290/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-290`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
