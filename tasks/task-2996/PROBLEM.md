# task-2996 — Transfer assessment · resume a partial frame

A decoder returns complete frames from a receive buffer. Add support for preserving an incomplete tail for the next read. Each header is a one-byte payload length, including zero. Return (complete payloads, unconsumed suffix starting at the incomplete header). The starter drops that suffix. Diagnose the earliest loss on two chunks that split one payload. Write a test that reassembles the split input using the returned suffix. Explain the copying versus lifetime tradeoff of returning owned bytes.

```rust
pub fn decode_batch(input: &[u8]) -> (Vec<Vec<u8>>, Vec<u8>)
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `decode_batch(&[])` gives `(Vec::<Vec<u8>>::new(),vec![])`.
- `decode_batch(&[2,4,1,0])` gives `(vec![vec![4,1], vec![]], vec![])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2996/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2996`.

[Module reading](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html)
