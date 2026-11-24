# task-1963 — Atomic bit union · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Start an AtomicU32 at zero. For each byte perform fetch_or with the byte widened to u32. Observe the value returned by the operation (the value preceding this update). Arithmetic wraps in u32. Explain why relaxed ordering suffices for this single-cell trace.

Return Some((minimum, maximum)) or None for an empty trace. Use AtomicU32 read-modify-write operations and Ordering::Relaxed. During review, point to the operation that enforces its contract.

```rust
pub fn atomic_bit_union_bounds(input: &[u8]) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `atomic_bit_union_bounds(&[])` gives `None`.
- `atomic_bit_union_bounds(&[4])` gives `Some((0u32, 0u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1963/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1963`.

[Module reading](https://doc.rust-lang.org/std/sync/atomic/)
