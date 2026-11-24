# task-2239 — Zero copy lines · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe lengths in bytes of logical lines, stripping LF or CRLF. Allocate only storage needed for the observations or result.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use borrowed slices or indices into the original input. During review, point to the operation that enforces its contract.

```rust
pub fn zero_copy_lines_longest_run(input: &str) -> u64
```

## Public examples

- `zero_copy_lines_longest_run("")` gives `0`.
- `zero_copy_lines_longest_run("a")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2239/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2239`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
