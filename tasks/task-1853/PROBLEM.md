# task-1853 — Shared read handles · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Start a fresh Arc<u8> per input byte. Observe the number of strong owners after making x modulo 4 more owners. Strong and weak handles have separate ownership semantics.

Return Some((minimum, maximum)) or None for an empty trace. Use Arc and explicit drop scopes. During review, point to the operation that enforces its contract.

```rust
pub fn shared_read_handles_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `shared_read_handles_bounds(&[])` gives `None`.
- `shared_read_handles_bounds(&[4])` gives `Some((1u32, 1u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1853/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1853`.

[Module reading](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
