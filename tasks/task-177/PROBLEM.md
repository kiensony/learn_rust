# task-177 — Three shelves · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Return 0 below 3, 1 from 3 through 5 inclusive, and 2 above 5.

```rust
pub fn three_shelves_3(first: u16, second: u16) -> u8
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `three_shelves_3(4, 0)` gives `1`.
- `three_shelves_3(13, 3)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-177/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-177`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
