# task-1941 — Locked batch peaks · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Publish one maximum for each consecutive nonempty chunk of 3 bytes into a mutex-protected journal. Compute each result before acquiring the lock. Observe the journal after ownership returns to the caller.

Return the observations in input order, including duplicates. Use std::sync::Mutex with a short critical section. During review, point to the operation that enforces its contract.

```rust
pub fn locked_batch_peaks_trace(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `locked_batch_peaks_trace(&[])` gives `vec![]`.
- `locked_batch_peaks_trace(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1941/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1941`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
