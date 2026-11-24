# task-2670 — Lending first byte · reverse

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Process consecutive borrowed chunks of up to 3 bytes. For each nonempty chunk observe the first byte. Do not allocate a copy of the chunk. During review connect the lifetime of each view with the API that produced it.

Return the observations in reverse order. Retain duplicate observations. Use a generic associated type tied to the receiver borrow. During review, point to the operation that enforces its contract.

```rust
pub fn lending_first_byte_reverse(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `lending_first_byte_reverse(&[])` gives `vec![]`.
- `lending_first_byte_reverse(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2670/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2670`.

[Module reading](https://doc.rust-lang.org/reference/items/associated-items.html)
