# task-166 — Override code · one reading

Allow 2, 4, and no other code.

```rust
pub fn override_code_2(value: u16) -> bool
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `override_code_2(4)` gives `true`.
- `override_code_2(13)` gives `false`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-166/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-166`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
