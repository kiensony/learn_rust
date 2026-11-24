# task-1859 — Shared read handles · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Start a fresh Arc<u8> per input byte. Observe the number of strong owners after making x modulo 4 more owners. Strong and weak handles have separate ownership semantics.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use Arc and explicit drop scopes. During review, point to the operation that enforces its contract.

```rust
pub fn shared_read_handles_longest_run(input: &[u8]) -> u64
```

## Public examples

- `shared_read_handles_longest_run(&[])` gives `0`.
- `shared_read_handles_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1859/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1859`.

[Module reading](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
