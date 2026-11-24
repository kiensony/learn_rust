# task-1882 — Unique owner state · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Start a fresh Arc<u8> per input byte. Observe one if a unique mutable reference is available while x modulo 4 other strong owners live, zero otherwise. Strong and weak handles have separate ownership semantics.

Return the sum of all observations, widening each one before adding. Use Arc and explicit drop scopes. During review, point to the operation that enforces its contract.

```rust
pub fn unique_owner_state_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `unique_owner_state_total(&[])` gives `0`.
- `unique_owner_state_total(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1882/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1882`.

[Module reading](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
