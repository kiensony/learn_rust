# task-2997 — Transfer assessment · reconcile a retry log

A delivery log contains (record ID, amount). A retry carries the same ID and amount. Produce a total by counting each ID once, widening amounts before summing. If an ID reappears with a different amount, stop at the earliest conflict and return Err(id). The starter masks conflicting retries. Reproduce with two interleaved IDs, add the smallest test proving that input order selects the reported conflict, and justify the map's cost for a long-lived service.

```rust
pub fn reconcile(events: &[(u16, u16)]) -> Result<u64, u16>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `reconcile(&[])` gives `Ok(0)`.
- `reconcile(&[(1,7),(2,5),(1,7)])` gives `Ok(12)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2997/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2997`.

[Module reading](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html)
