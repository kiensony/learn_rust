# task-1893 — Weak liveness · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Start a fresh Arc<u8> per input byte. Observe one if a weak handle upgrades after the original owner is dropped; retain one other owner exactly for odd bytes. Strong and weak handles have separate ownership semantics.

Return Some((minimum, maximum)) or None for an empty trace. Use Arc and explicit drop scopes. During review, point to the operation that enforces its contract.

```rust
pub fn weak_liveness_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `weak_liveness_bounds(&[])` gives `None`.
- `weak_liveness_bounds(&[4])` gives `Some((0u32, 0u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1893/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1893`.

[Module reading](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
