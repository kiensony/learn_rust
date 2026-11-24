# task-2569 — Odd element proof · longest run

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Exercise a disjoint-mutation contract. Make an owned copy. Mutate the entries beginning at zero-based offset 1, advancing by 2. Increment each visited byte with wrapping arithmetic and observe its new value. Give each entry a unique temporary mutable reference. Run the tests under the course's pinned Miri toolchain and explain why no two live mutation sites alias.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use a safe mutable-slice iterator and Miri. During review, point to the operation that enforces its contract.

```rust
pub fn odd_element_proof_longest_run(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `odd_element_proof_longest_run(&[])` gives `0`.
- `odd_element_proof_longest_run(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2569/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2569`.

[Module reading](https://github.com/rust-lang/miri)
