# task-131 — Whole cartons · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Return the fewest cartons of size 5 that can hold value pieces.

```rust
pub fn whole_cartons_5(issued: u16, returned: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `whole_cartons_5(4, 0)` gives `1`.
- `whole_cartons_5(13, 1)` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-131/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-131`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
