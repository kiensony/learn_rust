# task-1225 — Delimited fields · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Treat zero bytes as field terminators. Observe each field's byte length. Adjacent terminators produce an empty field and a trailing terminator finishes its field.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use BufRead::split. During review, point to the operation that enforces its contract.

```rust
pub fn delimited_fields_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `delimited_fields_changes(&[])` gives `vec![]`.
- `delimited_fields_changes(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1225/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1225`.

[Module reading](https://doc.rust-lang.org/std/io/)
