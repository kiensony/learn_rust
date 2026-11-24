# task-2230 — Borrowed scalars · reverse

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe UTF-8 byte offsets of Unicode scalar boundaries, excluding the final boundary. Allocate only storage needed for the observations or result.

Return the observations in reverse order. Retain duplicate observations. Use borrowed slices or indices into the original input. During review, point to the operation that enforces its contract.

```rust
pub fn borrowed_scalars_reverse(input: &str) -> Vec<u32>
```

## Public examples

- `borrowed_scalars_reverse("")` gives `vec![]`.
- `borrowed_scalars_reverse("a")` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2230/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2230`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
