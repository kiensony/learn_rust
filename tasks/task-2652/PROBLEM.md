# task-2652 — Lending windows · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Process consecutive borrowed chunks of up to 2 bytes. For each nonempty chunk observe the slice length. Do not allocate a copy of the chunk. During review connect the lifetime of each view with the API that produced it.

Return the sum of all observations, widening each one before adding. Use a generic associated type tied to the receiver borrow. During review, point to the operation that enforces its contract.

```rust
pub fn lending_windows_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `lending_windows_total(&[])` gives `0`.
- `lending_windows_total(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2652/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2652`.

[Module reading](https://doc.rust-lang.org/reference/items/associated-items.html)
