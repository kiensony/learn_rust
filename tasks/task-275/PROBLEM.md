# task-275 — Last field · stars

Return everything after the last '*' or all text when absent.

```rust
pub fn last_field_star(text: &str) -> String
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `last_field_star("cat")` gives `"cat"`.
- `last_field_star("*hi*")` gives `""`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-275/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-275`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
