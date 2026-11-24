# task-268 — Normalize spaces · hash marks

Split on whitespace and rejoin words with one '#' between adjacent words.

```rust
pub fn normalize_spaces_hash(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `normalize_spaces_hash("cat")` gives `"cat"`.
- `normalize_spaces_hash("#hi#")` gives `"#hi#"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-268/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-268`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
