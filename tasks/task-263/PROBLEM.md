# task-263 — Boundary trim · hash marks

Remove all consecutive '#' characters at either end.

```rust
pub fn boundary_trim_hash(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `boundary_trim_hash("cat")` gives `"cat"`.
- `boundary_trim_hash("#hi#")` gives `"hi"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-263/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-263`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
