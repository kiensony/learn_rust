# task-2999 — Transfer assessment · recover after queue closure

A producer accepts at most capacity records before downstream closes. Return both accepted and unaccepted payloads in original order, preserving ownership and every duplicate. Zero capacity closes immediately. Capacity greater than input length accepts all. The starter's shutdown path removes one undelivered record. Reproduce a loss adjacent to the cut. Add a conservation test that joins both outputs and recovers the exact input. Explain how the same accounting could be maintained by a bounded async channel.

```rust
pub fn handoff(input: Vec<u8>, capacity: usize) -> (Vec<u8>, Vec<u8>)
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `handoff(vec![4,5,6], 2)` gives `(vec![4,5],vec![6])`.
- `handoff(vec![9,9], 0)` gives `(vec![],vec![9,9])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2999/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2999`.

[Module reading](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html)
