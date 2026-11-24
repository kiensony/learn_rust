# task-929 — Consume a one-use fallback

Return the owned value when available, otherwise invoke the fallback once. A supplied value must skip the fallback entirely. Preserve support for a closure that consumes captured state.

```rust
pub fn provide(value: Option<String>, fallback: impl FnOnce() -> String) -> String
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `provide(Some(String::from("up")), || String::from("down"))` gives `"up"`.
- `provide(None, || String::from("late"))` gives `"late"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-929/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-929`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
