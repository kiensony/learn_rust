# task-301 — Remove a suffix · exclamation marks

Remove one trailing '!' if present. Keep the rest exactly.

```rust
pub fn remove_a_suffix_bang(text: &str) -> String
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `remove_a_suffix_bang("cat")` gives `"cat"`.
- `remove_a_suffix_bang("!hi!")` gives `"!hi"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-301/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-301`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
