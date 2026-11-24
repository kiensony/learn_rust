# task-277 — Count separators · stars

Return the decimal count of '*' in text as a String.

```rust
pub fn count_separators_star(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `count_separators_star("cat")` gives `"0"`.
- `count_separators_star("*hi*")` gives `"2"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-277/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-277`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
