# task-1854 — Shared read handles · peaks

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Start a fresh Arc<u8> per input byte. Observe the number of strong owners after making x modulo 4 more owners. Strong and weak handles have separate ownership semantics.

Keep the first observation and every later one strictly above all earlier observations. Use Arc and explicit drop scopes. During review, point to the operation that enforces its contract.

```rust
pub fn shared_read_handles_peaks(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shared_read_handles_peaks(&[])` gives `vec![]`.
- `shared_read_handles_peaks(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1854/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1854`.

[Module reading](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
