# task-1890 — Unique owner state · reverse

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Start a fresh Arc<u8> per input byte. Observe one if a unique mutable reference is available while x modulo 4 other strong owners live, zero otherwise. Strong and weak handles have separate ownership semantics.

Return the observations in reverse order. Retain duplicate observations. Use Arc and explicit drop scopes. During review, point to the operation that enforces its contract.

```rust
pub fn unique_owner_state_reverse(input: &[u8]) -> Vec<u32>
```

## Public examples

- `unique_owner_state_reverse(&[])` gives `vec![]`.
- `unique_owner_state_reverse(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1890/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1890`.

[Module reading](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
