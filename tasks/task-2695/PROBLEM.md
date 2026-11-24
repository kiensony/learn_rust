# task-2695 — Universal slice width · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Process consecutive borrowed chunks of up to 6 bytes. For each nonempty chunk observe the difference between greatest and least byte. Do not allocate a copy of the chunk. During review connect the lifetime of each view with the API that produced it.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use a higher-ranked bound for a projection callable on any temporary slice borrow. During review, point to the operation that enforces its contract.

```rust
pub fn universal_slice_width_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `universal_slice_width_changes(&[])` gives `vec![]`.
- `universal_slice_width_changes(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2695/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2695`.

[Module reading](https://doc.rust-lang.org/reference/items/associated-items.html)
