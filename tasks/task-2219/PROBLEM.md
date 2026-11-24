# task-2219 — Borrowed offsets · longest run

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe zero-based byte offsets of LF terminators. Allocate only storage needed for the observations or result.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use borrowed slices or indices into the original input. During review, point to the operation that enforces its contract.

```rust
pub fn borrowed_offsets_longest_run(input: &str) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `borrowed_offsets_longest_run("")` gives `0`.
- `borrowed_offsets_longest_run("a")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2219/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2219`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
