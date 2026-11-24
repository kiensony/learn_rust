# task-603 — Choose one of two borrowed labels

Select alert when urgent is true and normal otherwise. Borrow the selected input without allocation. Both candidate references participate in the output lifetime.

```rust
pub fn choose<'a>(urgent: bool, alert: &'a str, normal: &'a str) -> &'a str
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `choose(true, "stop", "go")` gives `"stop"`.
- `choose(false, "hold", "walk")` gives `"walk"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-603/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-603`.

[Module reading](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
