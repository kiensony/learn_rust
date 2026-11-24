# task-2680 — Lending checksum · reverse

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Process consecutive borrowed chunks of up to 4 bytes. For each nonempty chunk observe the bytewise XOR. Do not allocate a copy of the chunk. During review connect the lifetime of each view with the API that produced it.

Return the observations in reverse order. Retain duplicate observations. Use a generic associated type tied to the receiver borrow. During review, point to the operation that enforces its contract.

```rust
pub fn lending_checksum_reverse(input: &[u8]) -> Vec<u32>
```

## Public examples

- `lending_checksum_reverse(&[])` gives `vec![]`.
- `lending_checksum_reverse(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2680/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2680`.

[Module reading](https://doc.rust-lang.org/reference/items/associated-items.html)
