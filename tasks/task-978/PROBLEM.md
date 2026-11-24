# task-978 — Find the first negative reading

Return the zero-based index of the first negative reading. Return None if no negative reading occurs.

```rust
pub fn negative_at(values: &[i16]) -> Option<usize>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `negative_at(&[1, -2, -5])` gives `Some(1)`.
- `negative_at(&[0, 4])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-978/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-978`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
