# task-518 — Find a scalar at a character position

Return the scalar value at a zero-based Unicode scalar position. Combining marks count separately. An absent position yields None.

```rust
pub fn scalar_at(text: &str, position: usize) -> Option<char>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `scalar_at("café", 3)` gives `Some('é')`.
- `scalar_at("灯火", 1)` gives `Some('火')`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-518/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-518`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
