# task-2243 — Borrowed windows · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe the big-endian u16 represented by each overlapping two-byte window, including windows within a multibyte character. Allocate only storage needed for the observations or result.

Return Some((minimum, maximum)) or None for an empty trace. Use borrowed slices or indices into the original input. During review, point to the operation that enforces its contract.

```rust
pub fn borrowed_windows_bounds(input: &str) -> Option<(u32, u32)>
```

## Public examples

- `borrowed_windows_bounds("")` gives `None`.
- `borrowed_windows_bounds("a")` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2243/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2243`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
