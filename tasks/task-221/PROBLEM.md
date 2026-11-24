# task-221 — Sum chimes · two deliveries

Combine two deliveries, capping their sum at 255. Use the combined count as value. Sum the positive multiples of 3 no greater than value.

```rust
pub fn sum_chimes_3(first: u8, second: u8) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `sum_chimes_3(4, 0)` gives `3`.
- `sum_chimes_3(13, 3)` gives `45`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-221/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-221`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
