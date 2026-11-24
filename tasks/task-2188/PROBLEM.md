# task-2188 — Early exit probes · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Instrument an algorithm on a stream of keys. Observe the number of earlier entries inspected left-to-right, stopping at the first that is strictly greater than the current key, or after all earlier entries. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Return the absolute differences between each adjacent pair, in order. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn early_exit_probes_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `early_exit_probes_steps(&[])` gives `vec![]`.
- `early_exit_probes_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2188/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2188`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
