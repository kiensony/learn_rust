# task-1933 — A receiver waits for its own sender

A shutdown path sends every input byte to an unbounded std channel and then receives until disconnect. The starter retains its last sender in the draining scope. Reproduce the deterministic liveness failure and sketch the wait cycle. Return Ok with every enqueued byte in FIFO order, including duplicates. Keep recv_timeout as a guard that returns Err("stalled") after 25 ms of inactivity. The sender and receiver share one thread. Make shutdown complete by releasing the correct resource before waiting.

```rust
pub fn finish_queue(input: &[u8]) -> Result<Vec<u8>, &'static str>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `finish_queue(&[])` gives `Ok(vec![])`.
- `finish_queue(&[1,2])` gives `Ok(vec![1,2])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1933/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1933`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
