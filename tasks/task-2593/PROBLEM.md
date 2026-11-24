# task-2593 — Interior stride proof · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Exercise a disjoint-mutation contract. Make an owned copy. Mutate the entries beginning at zero-based offset 2, advancing by 3. Increment each visited byte with wrapping arithmetic and observe its new value. Give each entry a unique temporary mutable reference. Run the tests under the course's pinned Miri toolchain and explain why no two live mutation sites alias.

Return Some((minimum, maximum)) or None for an empty trace. Use a safe mutable-slice iterator and Miri. During review, point to the operation that enforces its contract.

```rust
pub fn interior_stride_proof_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `interior_stride_proof_bounds(&[])` gives `None`.
- `interior_stride_proof_bounds(&[4])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2593/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2593`.

[Module reading](https://github.com/rust-lang/miri)
