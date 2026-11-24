# task-2103 — Sorted keys · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A scheduler receives one u8 key per entry. Observe the distinct keys in sorted order. Explain the cost of inserting and extracting the next item.

Return Some((minimum, maximum)) or None for an empty trace. Use the standard library's BTreeSet. During review, point to the operation that enforces its contract.

```rust
pub fn sorted_keys_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `sorted_keys_bounds(&[])` gives `None`.
- `sorted_keys_bounds(&[4])` gives `Some((4u32, 4u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2103/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2103`.

[Module reading](https://doc.rust-lang.org/std/collections/)
