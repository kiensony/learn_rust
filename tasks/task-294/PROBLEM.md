# task-294 — Read a heading · plus signs

Return everything before the first '+' or all text when absent.

```rust
pub fn read_a_heading_plus(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `read_a_heading_plus("cat")` gives `"cat"`.
- `read_a_heading_plus("+hi+")` gives `""`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-294/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-294`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
