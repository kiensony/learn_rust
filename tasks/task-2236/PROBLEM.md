# task-2236 — Zero copy lines · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe lengths in bytes of logical lines, stripping LF or CRLF. Allocate only storage needed for the observations or result.

Count adjacent pairs for which the second observation is strictly smaller. Use borrowed slices or indices into the original input. During review, point to the operation that enforces its contract.

```rust
pub fn zero_copy_lines_descending_steps(input: &str) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `zero_copy_lines_descending_steps("")` gives `0`.
- `zero_copy_lines_descending_steps("a")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2236/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2236`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
