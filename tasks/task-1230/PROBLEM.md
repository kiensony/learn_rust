# task-1230 — Delimited fields · reverse

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Treat zero bytes as field terminators. Observe each field's byte length. Adjacent terminators produce an empty field and a trailing terminator finishes its field.

Return the observations in reverse order. Retain duplicate observations. Use BufRead::split. During review, point to the operation that enforces its contract.

```rust
pub fn delimited_fields_reverse(input: &[u8]) -> Vec<u32>
```

## Public examples

- `delimited_fields_reverse(&[])` gives `vec![]`.
- `delimited_fields_reverse(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1230/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1230`.

[Module reading](https://doc.rust-lang.org/std/io/)
