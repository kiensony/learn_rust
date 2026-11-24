# task-652 — Build a bounded stock record

Return a Stock with the original capacity and a count equal to original units plus amount, capped at capacity. An original count already over capacity is brought down to capacity.

```rust
pub fn restock(original: Stock, amount: u16) -> Stock
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `restock(Stock { units: 1, capacity: 8 }, 4)` gives `Stock { units: 5, capacity: 8 }`.
- `restock(Stock { units: 7, capacity: 8 }, 4)` gives `Stock { units: 8, capacity: 8 }`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-652/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-652`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
