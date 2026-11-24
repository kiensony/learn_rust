# task-567 — Borrow a label from the left record

Borrow left with its surrounding whitespace removed. The output may outlive the independent right argument. Keep the lifetime relationship in the signature.

```rust
pub fn left_caption<'a>(left: &'a str, _right: &str) -> &'a str
```

Implement the public function in `src/lib.rs`.

## Public examples

- `left_caption(" north ", "south")` gives `"north"`.
- `left_caption("w", "e")` gives `"w"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-567/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-567`.

[Module reading](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
