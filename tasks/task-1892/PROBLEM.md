# task-1892 — Weak liveness · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Start a fresh Arc<u8> per input byte. Observe one if a weak handle upgrades after the original owner is dropped; retain one other owner exactly for odd bytes. Strong and weak handles have separate ownership semantics.

Return the sum of all observations, widening each one before adding. Use Arc and explicit drop scopes. During review, point to the operation that enforces its contract.

```rust
pub fn weak_liveness_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `weak_liveness_total(&[])` gives `0`.
- `weak_liveness_total(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1892/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1892`.

[Module reading](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
