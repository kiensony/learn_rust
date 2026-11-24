# task-174 — Even batch · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Require an even value at least 3.

```rust
pub fn even_batch_3(first: u16, second: u16) -> bool
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `even_batch_3(4, 0)` gives `true`.
- `even_batch_3(4, 1)` gives `false`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-174/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-174`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
