# task-454 — Transfer between two counters

Move up to wanted units from account zero to account one. Leave both counters within u16 range, preserve their sum and return the actual amount moved.

```rust
pub fn transfer(accounts: &mut [u16; 2], wanted: u16) -> u16
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `{ let mut a = [10, 3]; let moved = transfer(&mut a, 4); (a,moved) }` gives `([6, 7], 4)`.
- `{ let mut a = [3, 2]; let moved = transfer(&mut a, 5); (a,moved) }` gives `([0, 5], 3)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-454/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-454`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
