# task-200 — Admission gate · limited supply

A request is limited by available stock. Use the smaller count as value. Admit values at least 10.

```rust
pub fn admission_gate_10(requested: u16, available: u16) -> bool
```

Implement the public function in `src/lib.rs`.

## Public examples

- `admission_gate_10(65535, 4)` gives `false`.
- `admission_gate_10(13, 14)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-200/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-200`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
