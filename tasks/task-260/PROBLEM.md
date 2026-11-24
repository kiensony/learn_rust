# task-260 — Return the width of a label

A function names a computation. Here the caller passes a borrowed piece of text named label and expects its byte length back. Call len() on label and return the result. The final expression in a Rust function supplies its return value.

```rust
pub fn width(label: &str) -> usize
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `width("box")` gives `3`.
- `width("postcard")` gives `8`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-260/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-260`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
