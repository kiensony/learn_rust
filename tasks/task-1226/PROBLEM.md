# task-1226 — Delimited fields · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Treat zero bytes as field terminators. Observe each field's byte length. Adjacent terminators produce an empty field and a trailing terminator finishes its field.

Count adjacent pairs for which the second observation is strictly smaller. Use BufRead::split. During review, point to the operation that enforces its contract.

```rust
pub fn delimited_fields_descending_steps(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `delimited_fields_descending_steps(&[])` gives `0`.
- `delimited_fields_descending_steps(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1226/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1226`.

[Module reading](https://doc.rust-lang.org/std/io/)
