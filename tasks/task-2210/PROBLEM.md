# task-2210 — Borrowed prefixes · reverse

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe byte lengths of the ASCII-whitespace-separated words. Allocate only storage needed for the observations or result.

Return the observations in reverse order. Retain duplicate observations. Use borrowed slices or indices into the original input. During review, point to the operation that enforces its contract.

```rust
pub fn borrowed_prefixes_reverse(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `borrowed_prefixes_reverse("")` gives `vec![]`.
- `borrowed_prefixes_reverse("a")` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2210/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2210`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
