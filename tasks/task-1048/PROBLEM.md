# task-1048 — Interleave fixed-size pairs

Pair elements at the same index in two arrays of equal compile-time length. Return an array with that same length. Consume both inputs without cloning or adding bounds on element types. Use the length invariant to justify each iterator access.

```rust
pub fn zip_array<A,B,const N: usize>(left: [A;N], right: [B;N]) -> [(A,B);N]
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `zip_array([2,4], ["a","b"])` gives `[(2,"a"),(4,"b")]`.
- `zip_array([true], [3])` gives `[(true,3)]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1048/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1048`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
