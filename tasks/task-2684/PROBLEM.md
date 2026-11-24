# task-2684 — Universal slice sum · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Process consecutive borrowed chunks of up to 5 bytes. For each nonempty chunk observe the sum of bytes. Do not allocate a copy of the chunk. During review connect the lifetime of each view with the API that produced it.

Keep the first observation and every later one strictly above all earlier observations. Use a higher-ranked bound for a projection callable on any temporary slice borrow. During review, point to the operation that enforces its contract.

```rust
pub fn universal_slice_sum_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `universal_slice_sum_peaks(&[])` gives `vec![]`.
- `universal_slice_sum_peaks(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2684/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2684`.

[Module reading](https://doc.rust-lang.org/reference/items/associated-items.html)
