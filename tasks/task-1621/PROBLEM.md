# task-1621 — Reproduce a flaky first winner

Two workers publish (latency, worker ID). Test scheduling can deliver their reports in either order. A consumer promises the ID of the fastest worker, breaking equal latency by lower worker ID. Return None for no reports. The schedule regression runs every ordering of a three-worker incident, without sleeping or retrying. Find the ordering that exposes the starter, repair the selection rule, and explain why running the original suite repeatedly could miss the failure.

```rust
pub fn fastest(reports: &[(u16, u8)]) -> Option<u8>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `fastest(&[])` gives `None`.
- `fastest(&[(12,2),(3,4)])` gives `Some(4)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1621/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1621`.

[Module reading](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
