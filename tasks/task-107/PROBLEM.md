# task-107 — Shadow the label with its size

Use shadowing in two steps. Rebind label to the trimmed label using trim(), then rebind it to the number of bytes using len(). Return the final number. A new let can reuse a name while changing its type.

```rust
pub fn label_size(label: &str) -> usize
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `label_size("  lamp")` gives `4`.
- `label_size("box ")` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-107/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-107`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
