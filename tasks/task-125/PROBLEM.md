# task-125 — Top up · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Return how many units to add to reach a multiple of 3. A full group needs zero.

```rust
pub fn top_up_3(first: u16, second: u16) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `top_up_3(4, 0)` gives `2`.
- `top_up_3(0, 6)` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-125/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-125`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
