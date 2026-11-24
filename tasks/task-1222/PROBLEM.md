# task-1222 — Delimited fields · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Treat zero bytes as field terminators. Observe each field's byte length. Adjacent terminators produce an empty field and a trailing terminator finishes its field.

Return the sum of all observations, widening each one before adding. Use BufRead::split. During review, point to the operation that enforces its contract.

```rust
pub fn delimited_fields_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `delimited_fields_total(&[])` gives `0`.
- `delimited_fields_total(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1222/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1222`.

[Module reading](https://doc.rust-lang.org/std/io/)
