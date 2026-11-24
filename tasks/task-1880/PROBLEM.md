# task-1880 — Shared payload · reverse

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Start a fresh Arc<u8> per input byte. Observe the byte visible through every owner. Strong and weak handles have separate ownership semantics.

Return the observations in reverse order. Retain duplicate observations. Use Arc and explicit drop scopes. During review, point to the operation that enforces its contract.

```rust
pub fn shared_payload_reverse(input: &[u8]) -> Vec<u32>
```

## Public examples

- `shared_payload_reverse(&[])` gives `vec![]`.
- `shared_payload_reverse(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1880/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1880`.

[Module reading](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
