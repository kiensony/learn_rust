# task-2110 — Sorted keys · reverse

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A scheduler receives one u8 key per entry. Observe the distinct keys in sorted order. Explain the cost of inserting and extracting the next item.

Return the observations in reverse order. Retain duplicate observations. Use the standard library's BTreeSet. During review, point to the operation that enforces its contract.

```rust
pub fn sorted_keys_reverse(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `sorted_keys_reverse(&[])` gives `vec![]`.
- `sorted_keys_reverse(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2110/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2110`.

[Module reading](https://doc.rust-lang.org/std/collections/)
