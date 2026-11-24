# task-272 — Boundary trim · stars

Remove all consecutive '*' characters at either end.

```rust
pub fn boundary_trim_star(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `boundary_trim_star("cat")` gives `"cat"`.
- `boundary_trim_star("*hi*")` gives `"hi"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-272/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-272`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
