# task-297 — Count separators · plus signs

Return the decimal count of '+' in text as a String.

```rust
pub fn count_separators_plus(text: &str) -> String
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `count_separators_plus("cat")` gives `"0"`.
- `count_separators_plus("+hi+")` gives `"2"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-297/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-297`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
