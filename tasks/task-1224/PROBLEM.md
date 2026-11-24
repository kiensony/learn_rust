# task-1224 — Delimited fields · peaks

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Treat zero bytes as field terminators. Observe each field's byte length. Adjacent terminators produce an empty field and a trailing terminator finishes its field.

Keep the first observation and every later one strictly above all earlier observations. Use BufRead::split. During review, point to the operation that enforces its contract.

```rust
pub fn delimited_fields_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `delimited_fields_peaks(&[])` gives `vec![]`.
- `delimited_fields_peaks(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1224/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1224`.

[Module reading](https://doc.rust-lang.org/std/io/)
