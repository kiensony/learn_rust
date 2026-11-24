# task-271 — Remove a suffix · stars

Remove one trailing '*' if present. Keep the rest exactly.

```rust
pub fn remove_a_suffix_star(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remove_a_suffix_star("cat")` gives `"cat"`.
- `remove_a_suffix_star("*hi*")` gives `"*hi"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-271/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-271`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
