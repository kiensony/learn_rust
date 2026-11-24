# task-2444 — Page alignment · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Each byte denotes a logical offset, unrelated to an actual allocation address. Compute the smallest offset at or above it that meets a slot type's alignment. Define the slot with repr(align(32)), query its alignment, and widen before rounding. Observe each aligned offset.

Keep the first observation and every later one strictly above all earlier observations. Use repr(align) and std::mem::align_of. During review, point to the operation that enforces its contract.

```rust
pub fn page_alignment_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `page_alignment_peaks(&[])` gives `vec![]`.
- `page_alignment_peaks(&[4])` gives `vec![32u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2444/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2444`.

[Module reading](https://doc.rust-lang.org/reference/type-layout.html)
