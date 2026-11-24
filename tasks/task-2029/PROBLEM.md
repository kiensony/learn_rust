# task-2029 — Polled groups · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Create a poll_fn future per byte. On its first poll wake_by_ref and return Pending. On its next poll return Ready. A two-poll harness can use a no-op waker because the second poll is explicit. Observe the ready payload, the byte divided by four, rounded down.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use a pinned future and an explicit polling context. During review, point to the operation that enforces its contract.

```rust
pub fn polled_groups_longest_run(input: &[u8]) -> u64
```

## Public examples

- `polled_groups_longest_run(&[])` gives `0`.
- `polled_groups_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2029/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2029`.

[Module reading](https://doc.rust-lang.org/std/future/)
