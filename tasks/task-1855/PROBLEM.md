# task-1855 — Shared read handles · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Start a fresh Arc<u8> per input byte. Observe the number of strong owners after making x modulo 4 more owners. Strong and weak handles have separate ownership semantics.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use Arc and explicit drop scopes. During review, point to the operation that enforces its contract.

```rust
pub fn shared_read_handles_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shared_read_handles_changes(&[])` gives `vec![]`.
- `shared_read_handles_changes(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1855/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1855`.

[Module reading](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
