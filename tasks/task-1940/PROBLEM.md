# task-1940 — Locked batch totals · reverse

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Publish one sum for each consecutive nonempty chunk of 2 bytes into a mutex-protected journal. Compute each result before acquiring the lock. Observe the journal after ownership returns to the caller.

Return the observations in reverse order. Retain duplicate observations. Use std::sync::Mutex with a short critical section. During review, point to the operation that enforces its contract.

```rust
pub fn locked_batch_totals_reverse(input: &[u8]) -> Vec<u32>
```

## Public examples

- `locked_batch_totals_reverse(&[])` gives `vec![]`.
- `locked_batch_totals_reverse(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1940/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1940`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
