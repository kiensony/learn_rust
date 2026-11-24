# task-2553 — Split proof · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Exercise a disjoint-mutation contract. Make an owned copy. Mutate the entries beginning at zero-based offset 0, advancing by 2. Increment each visited byte with wrapping arithmetic and observe its new value. Give each entry a unique temporary mutable reference. Run the tests under the course's pinned Miri toolchain and explain why no two live mutation sites alias.

Return Some((minimum, maximum)) or None for an empty trace. Use a safe mutable-slice iterator and Miri. During review, point to the operation that enforces its contract.

```rust
pub fn split_proof_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `split_proof_bounds(&[])` gives `None`.
- `split_proof_bounds(&[4])` gives `Some((5u32, 5u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2553/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2553`.

[Module reading](https://github.com/rust-lang/miri)
