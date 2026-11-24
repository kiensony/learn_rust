# task-434 — Keep the original first reading

Append exactly one reading and return the updated vector together with its original first value, if any. Reproduce E0502. End the shared borrow before the operation that can reallocate. Keep every original reading.

```rust
pub fn append_reading(mut readings: Vec<i32>, next: i32) -> (Vec<i32>, Option<i32>)
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `append_reading(vec![3, 6], 2)` gives `(vec![3, 6, 2], Some(3))`.
- `append_reading(vec![1], 4)` gives `(vec![1, 4], Some(1))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-434/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-434`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
