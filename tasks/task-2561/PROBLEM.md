# task-2561 — Odd element proof · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Exercise a disjoint-mutation contract. Make an owned copy. Mutate the entries beginning at zero-based offset 1, advancing by 2. Increment each visited byte with wrapping arithmetic and observe its new value. Give each entry a unique temporary mutable reference. Run the tests under the course's pinned Miri toolchain and explain why no two live mutation sites alias.

Return the observations in input order, including duplicates. Use a safe mutable-slice iterator and Miri. During review, point to the operation that enforces its contract.

```rust
pub fn odd_element_proof_trace(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `odd_element_proof_trace(&[])` gives `vec![]`.
- `odd_element_proof_trace(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2561/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2561`.

[Module reading](https://github.com/rust-lang/miri)
