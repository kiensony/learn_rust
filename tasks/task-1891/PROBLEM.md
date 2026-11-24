# task-1891 — Weak liveness · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Start a fresh Arc<u8> per input byte. Observe one if a weak handle upgrades after the original owner is dropped; retain one other owner exactly for odd bytes. Strong and weak handles have separate ownership semantics.

Return the observations in input order, including duplicates. Use Arc and explicit drop scopes. During review, point to the operation that enforces its contract.

```rust
pub fn weak_liveness_trace(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `weak_liveness_trace(&[])` gives `vec![]`.
- `weak_liveness_trace(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1891/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1891`.

[Module reading](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
