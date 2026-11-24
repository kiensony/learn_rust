# task-295 — Last field · plus signs

Return everything after the last '+' or all text when absent.

```rust
pub fn last_field_plus(text: &str) -> String
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `last_field_plus("cat")` gives `"cat"`.
- `last_field_plus("+hi+")` gives `""`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-295/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-295`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
