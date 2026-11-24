# task-1873 — Shared payload · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Start a fresh Arc<u8> per input byte. Observe the byte visible through every owner. Strong and weak handles have separate ownership semantics.

Return Some((minimum, maximum)) or None for an empty trace. Use Arc and explicit drop scopes. During review, point to the operation that enforces its contract.

```rust
pub fn shared_payload_bounds(input: &[u8]) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shared_payload_bounds(&[])` gives `None`.
- `shared_payload_bounds(&[4])` gives `Some((4u32, 4u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1873/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1873`.

[Module reading](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
