# task-2159 — Linear search probes · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Instrument an algorithm on a stream of keys. Observe the count of equality probes required by a forward linear search for each input key, including the successful probe. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn linear_search_probes_longest_run(input: &[u8]) -> u64
```

## Public examples

- `linear_search_probes_longest_run(&[])` gives `0`.
- `linear_search_probes_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2159/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2159`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
