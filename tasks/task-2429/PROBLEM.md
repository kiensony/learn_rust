# task-2429 — Wide alignment · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Each byte denotes a logical offset, unrelated to an actual allocation address. Compute the smallest offset at or above it that meets a slot type's alignment. Define the slot with repr(align(8)), query its alignment, and widen before rounding. Observe each aligned offset.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use repr(align) and std::mem::align_of. During review, point to the operation that enforces its contract.

```rust
pub fn wide_alignment_longest_run(input: &[u8]) -> u64
```

## Public examples

- `wide_alignment_longest_run(&[])` gives `0`.
- `wide_alignment_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2429/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2429`.

[Module reading](https://doc.rust-lang.org/reference/type-layout.html)
