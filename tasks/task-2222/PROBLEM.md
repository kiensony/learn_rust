# task-2222 — Borrowed scalars · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe UTF-8 byte offsets of Unicode scalar boundaries, excluding the final boundary. Allocate only storage needed for the observations or result.

Return the sum of all observations, widening each one before adding. Use borrowed slices or indices into the original input. During review, point to the operation that enforces its contract.

```rust
pub fn borrowed_scalars_total(input: &str) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `borrowed_scalars_total("")` gives `0`.
- `borrowed_scalars_total("a")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2222/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2222`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
