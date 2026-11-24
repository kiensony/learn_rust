# task-2565 — Odd element proof · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Exercise a disjoint-mutation contract. Make an owned copy. Mutate the entries beginning at zero-based offset 1, advancing by 2. Increment each visited byte with wrapping arithmetic and observe its new value. Give each entry a unique temporary mutable reference. Run the tests under the course's pinned Miri toolchain and explain why no two live mutation sites alias.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use a safe mutable-slice iterator and Miri. During review, point to the operation that enforces its contract.

```rust
pub fn odd_element_proof_changes(input: &[u8]) -> Vec<u32>
```

## Public examples

- `odd_element_proof_changes(&[])` gives `vec![]`.
- `odd_element_proof_changes(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2565/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2565`.

[Module reading](https://github.com/rust-lang/miri)
