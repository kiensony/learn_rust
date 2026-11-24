# task-2668 — Lending first byte · steps

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Process consecutive borrowed chunks of up to 3 bytes. For each nonempty chunk observe the first byte. Do not allocate a copy of the chunk. During review connect the lifetime of each view with the API that produced it.

Return the absolute differences between each adjacent pair, in order. Use a generic associated type tied to the receiver borrow. During review, point to the operation that enforces its contract.

```rust
pub fn lending_first_byte_steps(input: &[u8]) -> Vec<u32>
```

## Public examples

- `lending_first_byte_steps(&[])` gives `vec![]`.
- `lending_first_byte_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2668/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2668`.

[Module reading](https://doc.rust-lang.org/reference/items/associated-items.html)
