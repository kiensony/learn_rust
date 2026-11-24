# task-2554 — Split proof · peaks

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Exercise a disjoint-mutation contract. Make an owned copy. Mutate the entries beginning at zero-based offset 0, advancing by 2. Increment each visited byte with wrapping arithmetic and observe its new value. Give each entry a unique temporary mutable reference. Run the tests under the course's pinned Miri toolchain and explain why no two live mutation sites alias.

Keep the first observation and every later one strictly above all earlier observations. Use a safe mutable-slice iterator and Miri. During review, point to the operation that enforces its contract.

```rust
pub fn split_proof_peaks(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `split_proof_peaks(&[])` gives `vec![]`.
- `split_proof_peaks(&[4])` gives `vec![5u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2554/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2554`.

[Module reading](https://github.com/rust-lang/miri)
