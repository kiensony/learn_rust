# task-303 — Double a symbol · exclamation marks

Replace each '!' with two copies of '!'.

```rust
pub fn double_a_symbol_bang(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `double_a_symbol_bang("cat")` gives `"cat"`.
- `double_a_symbol_bang("!hi!")` gives `"!!hi!!"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-303/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-303`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
