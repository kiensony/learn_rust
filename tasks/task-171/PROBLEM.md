# task-171 — Last ticket · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Allow amounts strictly smaller than 3.

```rust
pub fn last_ticket_3(first: u16, second: u16) -> bool
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `last_ticket_3(4, 0)` gives `false`.
- `last_ticket_3(0, 0)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-171/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-171`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
