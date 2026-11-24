# task-1932 — Locked batch totals · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Publish one sum for each consecutive nonempty chunk of 2 bytes into a mutex-protected journal. Compute each result before acquiring the lock. Observe the journal after ownership returns to the caller.

Return the sum of all observations, widening each one before adding. Use std::sync::Mutex with a short critical section. During review, point to the operation that enforces its contract.

```rust
pub fn locked_batch_totals_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `locked_batch_totals_total(&[])` gives `0`.
- `locked_batch_totals_total(&[4])` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1932/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1932`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
