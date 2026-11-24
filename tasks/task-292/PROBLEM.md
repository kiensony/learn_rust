# task-292 — Boundary trim · plus signs

Remove all consecutive '+' characters at either end.

```rust
pub fn boundary_trim_plus(text: &str) -> String
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `boundary_trim_plus("cat")` gives `"cat"`.
- `boundary_trim_plus("+hi+")` gives `"hi"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-292/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-292`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
