# task-2672 — Lending checksum · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Process consecutive borrowed chunks of up to 4 bytes. For each nonempty chunk observe the bytewise XOR. Do not allocate a copy of the chunk. During review connect the lifetime of each view with the API that produced it.

Return the sum of all observations, widening each one before adding. Use a generic associated type tied to the receiver borrow. During review, point to the operation that enforces its contract.

```rust
pub fn lending_checksum_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `lending_checksum_total(&[])` gives `0`.
- `lending_checksum_total(&[4])` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2672/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2672`.

[Module reading](https://doc.rust-lang.org/reference/items/associated-items.html)
