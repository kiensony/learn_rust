# task-287 — Count separators · tildes

Return the decimal count of '~' in text as a String.

```rust
pub fn count_separators_tilde(text: &str) -> String
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `count_separators_tilde("cat")` gives `"0"`.
- `count_separators_tilde("~hi~")` gives `"2"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-287/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-287`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
