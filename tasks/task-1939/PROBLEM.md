# task-1939 — Locked batch totals · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Publish one sum for each consecutive nonempty chunk of 2 bytes into a mutex-protected journal. Compute each result before acquiring the lock. Observe the journal after ownership returns to the caller.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use std::sync::Mutex with a short critical section. During review, point to the operation that enforces its contract.

```rust
pub fn locked_batch_totals_longest_run(input: &[u8]) -> u64
```

## Public examples

- `locked_batch_totals_longest_run(&[])` gives `0`.
- `locked_batch_totals_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1939/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1939`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
