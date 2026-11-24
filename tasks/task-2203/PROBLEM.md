# task-2203 — Borrowed prefixes · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe byte lengths of the ASCII-whitespace-separated words. Allocate only storage needed for the observations or result.

Return Some((minimum, maximum)) or None for an empty trace. Use borrowed slices or indices into the original input. During review, point to the operation that enforces its contract.

```rust
pub fn borrowed_prefixes_bounds(input: &str) -> Option<(u32, u32)>
```

## Public examples

- `borrowed_prefixes_bounds("")` gives `None`.
- `borrowed_prefixes_bounds("a")` gives `Some((1u32, 1u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2203/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2203`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
