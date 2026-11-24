# task-2594 — Interior stride proof · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Exercise a disjoint-mutation contract. Make an owned copy. Mutate the entries beginning at zero-based offset 2, advancing by 3. Increment each visited byte with wrapping arithmetic and observe its new value. Give each entry a unique temporary mutable reference. Run the tests under the course's pinned Miri toolchain and explain why no two live mutation sites alias.

Keep the first observation and every later one strictly above all earlier observations. Use a safe mutable-slice iterator and Miri. During review, point to the operation that enforces its contract.

```rust
pub fn interior_stride_proof_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `interior_stride_proof_peaks(&[])` gives `vec![]`.
- `interior_stride_proof_peaks(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2594/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2594`.

[Module reading](https://github.com/rust-lang/miri)
