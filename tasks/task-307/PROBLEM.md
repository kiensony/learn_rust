# task-307 — Count separators · exclamation marks

Return the decimal count of '!' in text as a String.

```rust
pub fn count_separators_bang(text: &str) -> String
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `count_separators_bang("cat")` gives `"0"`.
- `count_separators_bang("!hi!")` gives `"2"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-307/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-307`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
