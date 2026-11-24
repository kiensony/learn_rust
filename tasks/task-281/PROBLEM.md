# task-281 — Remove a suffix · tildes

Remove one trailing '~' if present. Keep the rest exactly.

```rust
pub fn remove_a_suffix_tilde(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remove_a_suffix_tilde("cat")` gives `"cat"`.
- `remove_a_suffix_tilde("~hi~")` gives `"~hi"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-281/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-281`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
