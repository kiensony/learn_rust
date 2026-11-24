# task-273 — Double a symbol · stars

Replace each '*' with two copies of '*'.

```rust
pub fn double_a_symbol_star(text: &str) -> String
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `double_a_symbol_star("cat")` gives `"cat"`.
- `double_a_symbol_star("*hi*")` gives `"**hi**"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-273/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-273`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
