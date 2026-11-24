# task-2433 — Block alignment · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Each byte denotes a logical offset, unrelated to an actual allocation address. Compute the smallest offset at or above it that meets a slot type's alignment. Define the slot with repr(align(16)), query its alignment, and widen before rounding. Observe each aligned offset.

Return Some((minimum, maximum)) or None for an empty trace. Use repr(align) and std::mem::align_of. During review, point to the operation that enforces its contract.

```rust
pub fn block_alignment_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `block_alignment_bounds(&[])` gives `None`.
- `block_alignment_bounds(&[4])` gives `Some((16u32, 16u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2433/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2433`.

[Module reading](https://doc.rust-lang.org/reference/type-layout.html)
