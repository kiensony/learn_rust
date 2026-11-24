# task-1223 — Delimited fields · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Treat zero bytes as field terminators. Observe each field's byte length. Adjacent terminators produce an empty field and a trailing terminator finishes its field.

Return Some((minimum, maximum)) or None for an empty trace. Use BufRead::split. During review, point to the operation that enforces its contract.

```rust
pub fn delimited_fields_bounds(input: &[u8]) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `delimited_fields_bounds(&[])` gives `None`.
- `delimited_fields_bounds(&[4])` gives `Some((1u32, 1u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1223/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1223`.

[Module reading](https://doc.rust-lang.org/std/io/)
