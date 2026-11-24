# task-849 — Locate the first invalid dose

Return the sum of nonnegative doses as u64. At the first negative reading return a DoseError carrying its zero-based index and original signed value.

```rust
pub fn dose_total(doses: &[i8]) -> Result<u64, DoseError>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `dose_total(&[2,4,7])` gives `Ok(13)`.
- `dose_total(&[2,-4,-2])` gives `Err(DoseError { index: 1, value: -4 })`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-849/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-849`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
