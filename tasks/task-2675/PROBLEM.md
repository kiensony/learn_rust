# task-2675 — Lending checksum · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Process consecutive borrowed chunks of up to 4 bytes. For each nonempty chunk observe the bytewise XOR. Do not allocate a copy of the chunk. During review connect the lifetime of each view with the API that produced it.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use a generic associated type tied to the receiver borrow. During review, point to the operation that enforces its contract.

```rust
pub fn lending_checksum_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `lending_checksum_changes(&[])` gives `vec![]`.
- `lending_checksum_changes(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2675/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2675`.

[Module reading](https://doc.rust-lang.org/reference/items/associated-items.html)
