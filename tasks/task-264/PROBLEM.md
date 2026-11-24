# task-264 — Double a symbol · hash marks

Replace each '#' with two copies of '#'.

```rust
pub fn double_a_symbol_hash(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `double_a_symbol_hash("cat")` gives `"cat"`.
- `double_a_symbol_hash("#hi#")` gives `"##hi##"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-264/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-264`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
