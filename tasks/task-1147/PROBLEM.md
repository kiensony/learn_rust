# task-1147 — Chain formatting policies at runtime

Apply each Filter in sequence, feeding its result to the next. Support mixed types and an empty pipeline. The final result must preserve the declared order.

```rust
pub fn apply_all(mut value: i16, steps: &[&dyn Filter]) -> i16
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `apply_all(3, &[&Offset(2), &Clamp(4)])` gives `4`.
- `apply_all(3, &[&Clamp(4), &Offset(2)])` gives `5`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1147/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1147`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)
