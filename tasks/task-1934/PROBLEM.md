# task-1934 — Locked batch totals · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Publish one sum for each consecutive nonempty chunk of 2 bytes into a mutex-protected journal. Compute each result before acquiring the lock. Observe the journal after ownership returns to the caller.

Keep the first observation and every later one strictly above all earlier observations. Use std::sync::Mutex with a short critical section. During review, point to the operation that enforces its contract.

```rust
pub fn locked_batch_totals_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `locked_batch_totals_peaks(&[])` gives `vec![]`.
- `locked_batch_totals_peaks(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1934/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1934`.

[Module reading](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
