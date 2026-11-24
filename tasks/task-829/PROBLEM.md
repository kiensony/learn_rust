# task-829 — Propagate the first failed field

Add both successful values using u16. When the left result failed, return its error first regardless of the right value. Otherwise propagate the right error unchanged.

```rust
pub fn combine(left: Result<u8, Field>, right: Result<u8, Field>) -> Result<u16, Field>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `combine(Ok(3), Ok(2))` gives `Ok(5)`.
- `combine(Err(Field::Left), Err(Field::Right))` gives `Err(Field::Left)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-829/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-829`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
