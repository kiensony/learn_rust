# task-162 — Choose a larger score

Compare the two scores with > in an if expression. Return the larger score. Equal scores return their common value. Negative scores are valid.

```rust
pub fn higher(first: i16, second: i16) -> i16
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `higher(2, 7)` gives `7`.
- `higher(0, 0)` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-162/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-162`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
