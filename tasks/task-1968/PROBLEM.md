# task-1968 — Atomic bit union · steps

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Start an AtomicU32 at zero. For each byte perform fetch_or with the byte widened to u32. Observe the value returned by the operation (the value preceding this update). Arithmetic wraps in u32. Explain why relaxed ordering suffices for this single-cell trace.

Return the absolute differences between each adjacent pair, in order. Use AtomicU32 read-modify-write operations and Ordering::Relaxed. During review, point to the operation that enforces its contract.

```rust
pub fn atomic_bit_union_steps(input: &[u8]) -> Vec<u32>
```

## Public examples

- `atomic_bit_union_steps(&[])` gives `vec![]`.
- `atomic_bit_union_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1968/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1968`.

[Module reading](https://doc.rust-lang.org/std/sync/atomic/)
