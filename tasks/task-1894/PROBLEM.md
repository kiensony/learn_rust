# task-1894 — Weak liveness · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Start a fresh Arc<u8> per input byte. Observe one if a weak handle upgrades after the original owner is dropped; retain one other owner exactly for odd bytes. Strong and weak handles have separate ownership semantics.

Keep the first observation and every later one strictly above all earlier observations. Use Arc and explicit drop scopes. During review, point to the operation that enforces its contract.

```rust
pub fn weak_liveness_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `weak_liveness_peaks(&[])` gives `vec![]`.
- `weak_liveness_peaks(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1894/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1894`.

[Module reading](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
