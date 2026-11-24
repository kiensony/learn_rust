# task-2653 — Lending windows · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Process consecutive borrowed chunks of up to 2 bytes. For each nonempty chunk observe the slice length. Do not allocate a copy of the chunk. During review connect the lifetime of each view with the API that produced it.

Return Some((minimum, maximum)) or None for an empty trace. Use a generic associated type tied to the receiver borrow. During review, point to the operation that enforces its contract.

```rust
pub fn lending_windows_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `lending_windows_bounds(&[])` gives `None`.
- `lending_windows_bounds(&[4])` gives `Some((1u32, 1u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2653/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2653`.

[Module reading](https://doc.rust-lang.org/reference/items/associated-items.html)
