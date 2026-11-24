# task-554 — Count the occupied columns of a puzzle

Each scalar is one cell in this puzzle. A middle dot (·) marks an empty cell. Return (occupied, empty). Combining marks each occupy their own cell.

```rust
pub fn puzzle_cells(row: &str) -> (usize, usize)
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `puzzle_cells("A·Z")` gives `(2, 1)`.
- `puzzle_cells("猫··虎")` gives `(2, 2)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-554/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-554`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
