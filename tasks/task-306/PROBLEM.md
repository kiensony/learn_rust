# task-306 — Blank notice · exclamation marks

Return '!' if text is empty after trimming whitespace. Otherwise return trimmed text.

```rust
pub fn blank_notice_bang(text: &str) -> String
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `blank_notice_bang("cat")` gives `"cat"`.
- `blank_notice_bang("!hi!")` gives `"!hi!"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-306/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-306`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
