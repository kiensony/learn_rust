# task-1869 — Weak read handles · longest run

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Start a fresh Arc<u8> per input byte. Observe the number of explicit weak observers after making x modulo 4 observers. Strong and weak handles have separate ownership semantics.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use Arc and explicit drop scopes. During review, point to the operation that enforces its contract.

```rust
pub fn weak_read_handles_longest_run(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `weak_read_handles_longest_run(&[])` gives `0`.
- `weak_read_handles_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1869/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1869`.

[Module reading](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
