# task-2413 — Word alignment · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Each byte denotes a logical offset, unrelated to an actual allocation address. Compute the smallest offset at or above it that meets a slot type's alignment. Define the slot with repr(align(4)), query its alignment, and widen before rounding. Observe each aligned offset.

Return Some((minimum, maximum)) or None for an empty trace. Use repr(align) and std::mem::align_of. During review, point to the operation that enforces its contract.

```rust
pub fn word_alignment_bounds(input: &[u8]) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `word_alignment_bounds(&[])` gives `None`.
- `word_alignment_bounds(&[4])` gives `Some((4u32, 4u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2413/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2413`.

[Module reading](https://doc.rust-lang.org/reference/type-layout.html)
