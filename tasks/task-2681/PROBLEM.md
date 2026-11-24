# task-2681 — Universal slice sum · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Process consecutive borrowed chunks of up to 5 bytes. For each nonempty chunk observe the sum of bytes. Do not allocate a copy of the chunk. During review connect the lifetime of each view with the API that produced it.

Return the observations in input order, including duplicates. Use a higher-ranked bound for a projection callable on any temporary slice borrow. During review, point to the operation that enforces its contract.

```rust
pub fn universal_slice_sum_trace(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `universal_slice_sum_trace(&[])` gives `vec![]`.
- `universal_slice_sum_trace(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2681/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2681`.

[Module reading](https://doc.rust-lang.org/reference/items/associated-items.html)
