# task-2233 — Zero copy lines · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe lengths in bytes of logical lines, stripping LF or CRLF. Allocate only storage needed for the observations or result.

Return Some((minimum, maximum)) or None for an empty trace. Use borrowed slices or indices into the original input. During review, point to the operation that enforces its contract.

```rust
pub fn zero_copy_lines_bounds(input: &str) -> Option<(u32, u32)>
```

## Public examples

- `zero_copy_lines_bounds("")` gives `None`.
- `zero_copy_lines_bounds("a")` gives `Some((1u32, 1u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2233/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2233`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
