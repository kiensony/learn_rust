# task-1862 — Weak read handles · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Start a fresh Arc<u8> per input byte. Observe the number of explicit weak observers after making x modulo 4 observers. Strong and weak handles have separate ownership semantics.

Return the sum of all observations, widening each one before adding. Use Arc and explicit drop scopes. During review, point to the operation that enforces its contract.

```rust
pub fn weak_read_handles_total(input: &[u8]) -> u64
```

## Public examples

- `weak_read_handles_total(&[])` gives `0`.
- `weak_read_handles_total(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1862/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1862`.

[Module reading](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
