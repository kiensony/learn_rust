# task-1959 — Atomic running total · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Start an AtomicU32 at zero. For each byte perform fetch_add with the byte widened to u32. Observe the value returned by the operation (the value preceding this update). Arithmetic wraps in u32. Explain why relaxed ordering suffices for this single-cell trace.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use AtomicU32 read-modify-write operations and Ordering::Relaxed. During review, point to the operation that enforces its contract.

```rust
pub fn atomic_running_total_longest_run(input: &[u8]) -> u64
```

## Public examples

- `atomic_running_total_longest_run(&[])` gives `0`.
- `atomic_running_total_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1959/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1959`.

[Module reading](https://doc.rust-lang.org/std/sync/atomic/)
