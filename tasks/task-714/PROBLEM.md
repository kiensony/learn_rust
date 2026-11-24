# task-714 — Combine optional measurements

Return second minus first when both readings exist. Widen before subtraction. Either missing reading makes the result None.

```rust
pub fn difference(first: Option<i16>, second: Option<i16>) -> Option<i32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `difference(Some(2), Some(8))` gives `Some(6)`.
- `difference(Some(3), Some(-4))` gives `Some(-7)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-714/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-714`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
