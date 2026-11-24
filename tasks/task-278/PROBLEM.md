# task-278 — Normalize spaces · stars

Split on whitespace and rejoin words with one '*' between adjacent words.

```rust
pub fn normalize_spaces_star(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `normalize_spaces_star("cat")` gives `"cat"`.
- `normalize_spaces_star("*hi*")` gives `"*hi*"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-278/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-278`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
