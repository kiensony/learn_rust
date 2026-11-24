# task-1943 — Locked batch peaks · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Publish one maximum for each consecutive nonempty chunk of 3 bytes into a mutex-protected journal. Compute each result before acquiring the lock. Observe the journal after ownership returns to the caller.

Return Some((minimum, maximum)) or None for an empty trace. Use std::sync::Mutex with a short critical section. During review, point to the operation that enforces its contract.

```rust
pub fn locked_batch_peaks_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `locked_batch_peaks_bounds(&[])` gives `None`.
- `locked_batch_peaks_bounds(&[4])` gives `Some((4u32, 4u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1943/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1943`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
