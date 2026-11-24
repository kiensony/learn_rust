# task-265 — Last field · hash marks

Return everything after the last '#' or all text when absent.

```rust
pub fn last_field_hash(text: &str) -> String
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `last_field_hash("cat")` gives `"cat"`.
- `last_field_hash("#hi#")` gives `""`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-265/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-265`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
