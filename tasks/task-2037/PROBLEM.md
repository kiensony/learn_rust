# task-2037 — A pending future forgets its consumer

A countdown future yields Pending exactly delays times, then Ready. Each Pending transition schedules exactly one new poll through the registered waker. A bounded harness polls explicitly even when the wake is missing. Return (poll count including Ready, wake notifications). Use a real Waker counter, keep the same pinned future between polls, and avoid sleeping. Explain why a production executor could stop making progress with the starter.

```rust
pub fn poll_gate(delays: u8) -> (u16, u16)
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `poll_gate(0)` gives `(1,0)`.
- `poll_gate(1)` gives `(2,1)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2037/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2037`.

[Module reading](https://doc.rust-lang.org/std/future/)
