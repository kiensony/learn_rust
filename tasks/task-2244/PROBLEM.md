# task-2244 — Borrowed windows · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe the big-endian u16 represented by each overlapping two-byte window, including windows within a multibyte character. Allocate only storage needed for the observations or result.

Keep the first observation and every later one strictly above all earlier observations. Use borrowed slices or indices into the original input. During review, point to the operation that enforces its contract.

```rust
pub fn borrowed_windows_peaks(input: &str) -> Vec<u32>
```

## Public examples

- `borrowed_windows_peaks("")` gives `vec![]`.
- `borrowed_windows_peaks("a")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2244/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2244`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
