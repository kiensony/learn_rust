# task-266 — Blank notice · hash marks

Return '#' if text is empty after trimming whitespace. Otherwise return trimmed text.

```rust
pub fn blank_notice_hash(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `blank_notice_hash("cat")` gives `"cat"`.
- `blank_notice_hash("#hi#")` gives `"#hi#"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-266/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-266`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
