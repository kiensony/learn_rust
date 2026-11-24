# task-280 — Remove a prefix · tildes

Remove one leading '~' if present. Keep the rest exactly.

```rust
pub fn remove_a_prefix_tilde(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remove_a_prefix_tilde("cat")` gives `"cat"`.
- `remove_a_prefix_tilde("~hi~")` gives `"hi~"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-280/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-280`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
