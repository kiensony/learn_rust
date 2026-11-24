# task-2242 — Borrowed windows · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe the big-endian u16 represented by each overlapping two-byte window, including windows within a multibyte character. Allocate only storage needed for the observations or result.

Return the sum of all observations, widening each one before adding. Use borrowed slices or indices into the original input. During review, point to the operation that enforces its contract.

```rust
pub fn borrowed_windows_total(input: &str) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `borrowed_windows_total("")` gives `0`.
- `borrowed_windows_total("a")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2242/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2242`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
