# task-1953 — Atomic running total · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Start an AtomicU32 at zero. For each byte perform fetch_add with the byte widened to u32. Observe the value returned by the operation (the value preceding this update). Arithmetic wraps in u32. Explain why relaxed ordering suffices for this single-cell trace.

Return Some((minimum, maximum)) or None for an empty trace. Use AtomicU32 read-modify-write operations and Ordering::Relaxed. During review, point to the operation that enforces its contract.

```rust
pub fn atomic_running_total_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `atomic_running_total_bounds(&[])` gives `None`.
- `atomic_running_total_bounds(&[4])` gives `Some((0u32, 0u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1953/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1953`.

[Module reading](https://doc.rust-lang.org/std/sync/atomic/)
