# task-283 — Double a symbol · tildes

Replace each '~' with two copies of '~'.

```rust
pub fn double_a_symbol_tilde(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `double_a_symbol_tilde("cat")` gives `"cat"`.
- `double_a_symbol_tilde("~hi~")` gives `"~~hi~~"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-283/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-283`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
