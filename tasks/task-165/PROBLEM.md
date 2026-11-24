# task-165 — Even batch · one reading

Require an even value at least 2.

```rust
pub fn even_batch_2(value: u16) -> bool
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `even_batch_2(4)` gives `true`.
- `even_batch_2(13)` gives `false`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-165/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-165`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
