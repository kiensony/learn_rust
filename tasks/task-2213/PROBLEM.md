# task-2213 — Borrowed offsets · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe zero-based byte offsets of LF terminators. Allocate only storage needed for the observations or result.

Return Some((minimum, maximum)) or None for an empty trace. Use borrowed slices or indices into the original input. During review, point to the operation that enforces its contract.

```rust
pub fn borrowed_offsets_bounds(input: &str) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `borrowed_offsets_bounds("")` gives `None`.
- `borrowed_offsets_bounds("a")` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2213/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2213`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
