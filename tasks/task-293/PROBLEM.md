# task-293 — Double a symbol · plus signs

Replace each '+' with two copies of '+'.

```rust
pub fn double_a_symbol_plus(text: &str) -> String
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `double_a_symbol_plus("cat")` gives `"cat"`.
- `double_a_symbol_plus("+hi+")` gives `"++hi++"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-293/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-293`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
