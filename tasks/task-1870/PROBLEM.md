# task-1870 — Weak read handles · reverse

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Start a fresh Arc<u8> per input byte. Observe the number of explicit weak observers after making x modulo 4 observers. Strong and weak handles have separate ownership semantics.

Return the observations in reverse order. Retain duplicate observations. Use Arc and explicit drop scopes. During review, point to the operation that enforces its contract.

```rust
pub fn weak_read_handles_reverse(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `weak_read_handles_reverse(&[])` gives `vec![]`.
- `weak_read_handles_reverse(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1870/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1870`.

[Module reading](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
