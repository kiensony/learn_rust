# task-2435 — Block alignment · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Each byte denotes a logical offset, unrelated to an actual allocation address. Compute the smallest offset at or above it that meets a slot type's alignment. Define the slot with repr(align(16)), query its alignment, and widen before rounding. Observe each aligned offset.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use repr(align) and std::mem::align_of. During review, point to the operation that enforces its contract.

```rust
pub fn block_alignment_changes(input: &[u8]) -> Vec<u32>
```

## Public examples

- `block_alignment_changes(&[])` gives `vec![]`.
- `block_alignment_changes(&[4])` gives `vec![16u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2435/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2435`.

[Module reading](https://doc.rust-lang.org/reference/type-layout.html)
