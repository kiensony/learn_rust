# task-2689 — Universal slice sum · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Process consecutive borrowed chunks of up to 5 bytes. For each nonempty chunk observe the sum of bytes. Do not allocate a copy of the chunk. During review connect the lifetime of each view with the API that produced it.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use a higher-ranked bound for a projection callable on any temporary slice borrow. During review, point to the operation that enforces its contract.

```rust
pub fn universal_slice_sum_longest_run(input: &[u8]) -> u64
```

## Public examples

- `universal_slice_sum_longest_run(&[])` gives `0`.
- `universal_slice_sum_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2689/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2689`.

[Module reading](https://doc.rust-lang.org/reference/items/associated-items.html)
