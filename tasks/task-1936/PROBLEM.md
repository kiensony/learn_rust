# task-1936 — Locked batch totals · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Publish one sum for each consecutive nonempty chunk of 2 bytes into a mutex-protected journal. Compute each result before acquiring the lock. Observe the journal after ownership returns to the caller.

Count adjacent pairs for which the second observation is strictly smaller. Use std::sync::Mutex with a short critical section. During review, point to the operation that enforces its contract.

```rust
pub fn locked_batch_totals_descending_steps(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `locked_batch_totals_descending_steps(&[])` gives `0`.
- `locked_batch_totals_descending_steps(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1936/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1936`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
