# task-1228 — Delimited fields · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Treat zero bytes as field terminators. Observe each field's byte length. Adjacent terminators produce an empty field and a trailing terminator finishes its field.

Return the absolute differences between each adjacent pair, in order. Use BufRead::split. During review, point to the operation that enforces its contract.

```rust
pub fn delimited_fields_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `delimited_fields_steps(&[])` gives `vec![]`.
- `delimited_fields_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1228/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1228`.

[Module reading](https://doc.rust-lang.org/std/io/)
