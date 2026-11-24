# task-2424 — Wide alignment · peaks

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Each byte denotes a logical offset, unrelated to an actual allocation address. Compute the smallest offset at or above it that meets a slot type's alignment. Define the slot with repr(align(8)), query its alignment, and widen before rounding. Observe each aligned offset.

Keep the first observation and every later one strictly above all earlier observations. Use repr(align) and std::mem::align_of. During review, point to the operation that enforces its contract.

```rust
pub fn wide_alignment_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `wide_alignment_peaks(&[])` gives `vec![]`.
- `wide_alignment_peaks(&[4])` gives `vec![8u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2424/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2424`.

[Module reading](https://doc.rust-lang.org/reference/type-layout.html)
