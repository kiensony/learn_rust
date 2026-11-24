# task-233 — Running deposits · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Deposit 5 more coins each day than the day before, starting with 5 on day one. Return total for value days.

```rust
pub fn running_deposits_5(issued: u8, returned: u8) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `running_deposits_5(4, 0)` gives `50`.
- `running_deposits_5(13, 1)` gives `390`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-233/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-233`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
