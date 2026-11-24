# task-2129 — Queue rotation · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

A scheduler receives one u8 key per entry. Observe the FIFO order after moving the front element to the back exactly once, when present. Explain the cost of inserting and extracting the next item.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use the standard library's VecDeque. During review, point to the operation that enforces its contract.

```rust
pub fn queue_rotation_longest_run(input: &[u8]) -> u64
```

## Public examples

- `queue_rotation_longest_run(&[])` gives `0`.
- `queue_rotation_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2129/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2129`.

[Module reading](https://doc.rust-lang.org/std/collections/)
