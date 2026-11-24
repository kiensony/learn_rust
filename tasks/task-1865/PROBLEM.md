# task-1865 — Weak read handles · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Start a fresh Arc<u8> per input byte. Observe the number of explicit weak observers after making x modulo 4 observers. Strong and weak handles have separate ownership semantics.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use Arc and explicit drop scopes. During review, point to the operation that enforces its contract.

```rust
pub fn weak_read_handles_changes(input: &[u8]) -> Vec<u32>
```

## Public examples

- `weak_read_handles_changes(&[])` gives `vec![]`.
- `weak_read_handles_changes(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1865/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1865`.

[Module reading](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
