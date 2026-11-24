# task-279 — Wrap a message · tildes

Return text surrounded on both sides by '~'.

```rust
pub fn wrap_a_message_tilde(text: &str) -> String
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `wrap_a_message_tilde("cat")` gives `"~cat~"`.
- `wrap_a_message_tilde("~hi~")` gives `"~~hi~~"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-279/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-279`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
