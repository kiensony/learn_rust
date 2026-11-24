# task-2404 — Header alignment · peaks

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Each byte denotes a logical offset, unrelated to an actual allocation address. Compute the smallest offset at or above it that meets a slot type's alignment. Define the slot with repr(align(2)), query its alignment, and widen before rounding. Observe each aligned offset.

Keep the first observation and every later one strictly above all earlier observations. Use repr(align) and std::mem::align_of. During review, point to the operation that enforces its contract.

```rust
pub fn header_alignment_peaks(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `header_alignment_peaks(&[])` gives `vec![]`.
- `header_alignment_peaks(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2404/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2404`.

[Module reading](https://doc.rust-lang.org/reference/type-layout.html)
