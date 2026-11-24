# task-304 — Read a heading · exclamation marks

Return everything before the first '!' or all text when absent.

```rust
pub fn read_a_heading_bang(text: &str) -> String
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `read_a_heading_bang("cat")` gives `"cat"`.
- `read_a_heading_bang("!hi!")` gives `""`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-304/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-304`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
