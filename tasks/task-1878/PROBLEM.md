# task-1878 — Shared payload · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Start a fresh Arc<u8> per input byte. Observe the byte visible through every owner. Strong and weak handles have separate ownership semantics.

Return the absolute differences between each adjacent pair, in order. Use Arc and explicit drop scopes. During review, point to the operation that enforces its contract.

```rust
pub fn shared_payload_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shared_payload_steps(&[])` gives `vec![]`.
- `shared_payload_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1878/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1878`.

[Module reading](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
