# task-298 — Normalize spaces · plus signs

Split on whitespace and rejoin words with one '+' between adjacent words.

```rust
pub fn normalize_spaces_plus(text: &str) -> String
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `normalize_spaces_plus("cat")` gives `"cat"`.
- `normalize_spaces_plus("+hi+")` gives `"+hi+"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-298/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-298`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
