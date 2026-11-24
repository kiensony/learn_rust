# task-2659 — Lending windows · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Process consecutive borrowed chunks of up to 2 bytes. For each nonempty chunk observe the slice length. Do not allocate a copy of the chunk. During review connect the lifetime of each view with the API that produced it.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use a generic associated type tied to the receiver borrow. During review, point to the operation that enforces its contract.

```rust
pub fn lending_windows_longest_run(input: &[u8]) -> u64
```

## Public examples

- `lending_windows_longest_run(&[])` gives `0`.
- `lending_windows_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2659/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2659`.

[Module reading](https://doc.rust-lang.org/reference/items/associated-items.html)
