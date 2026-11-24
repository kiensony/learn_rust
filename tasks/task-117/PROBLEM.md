# task-117 — Fit a byte · one reading

Add 2 to value and return the result capped at 255.

```rust
pub fn fit_a_byte_2(value: u16) -> u32
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `fit_a_byte_2(4)` gives `6`.
- `fit_a_byte_2(13)` gives `15`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-117/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-117`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
