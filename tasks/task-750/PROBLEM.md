# task-750 — Select the first available sample

Return the value of the first successful sample in input order. Skip failed samples. If no reading succeeds, return None.

```rust
pub fn first_ok(samples: &[Result<i16, ()>]) -> Option<i16>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `first_ok(&[Err(()), Ok(8), Ok(9)])` gives `Some(8)`.
- `first_ok(&[Ok(-2)])` gives `Some(-2)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-750/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-750`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
