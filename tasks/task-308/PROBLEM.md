# task-308 — Normalize spaces · exclamation marks

Split on whitespace and rejoin words with one '!' between adjacent words.

```rust
pub fn normalize_spaces_bang(text: &str) -> String
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `normalize_spaces_bang("cat")` gives `"cat"`.
- `normalize_spaces_bang("!hi!")` gives `"!hi!"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-308/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-308`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
