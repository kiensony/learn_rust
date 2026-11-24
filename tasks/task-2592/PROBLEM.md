# task-2592 — Interior stride proof · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Exercise a disjoint-mutation contract. Make an owned copy. Mutate the entries beginning at zero-based offset 2, advancing by 3. Increment each visited byte with wrapping arithmetic and observe its new value. Give each entry a unique temporary mutable reference. Run the tests under the course's pinned Miri toolchain and explain why no two live mutation sites alias.

Return the sum of all observations, widening each one before adding. Use a safe mutable-slice iterator and Miri. During review, point to the operation that enforces its contract.

```rust
pub fn interior_stride_proof_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `interior_stride_proof_total(&[])` gives `0`.
- `interior_stride_proof_total(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2592/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2592`.

[Module reading](https://github.com/rust-lang/miri)
