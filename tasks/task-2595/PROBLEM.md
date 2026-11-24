# task-2595 — Interior stride proof · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Exercise a disjoint-mutation contract. Make an owned copy. Mutate the entries beginning at zero-based offset 2, advancing by 3. Increment each visited byte with wrapping arithmetic and observe its new value. Give each entry a unique temporary mutable reference. Run the tests under the course's pinned Miri toolchain and explain why no two live mutation sites alias.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use a safe mutable-slice iterator and Miri. During review, point to the operation that enforces its contract.

```rust
pub fn interior_stride_proof_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `interior_stride_proof_changes(&[])` gives `vec![]`.
- `interior_stride_proof_changes(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2595/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2595`.

[Module reading](https://github.com/rust-lang/miri)
