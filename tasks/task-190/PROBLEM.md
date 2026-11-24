# task-190 — Admission gate · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Admit values at least 8.

```rust
pub fn admission_gate_8(start: u16, end: u16) -> bool
```

Implement the public function in `src/lib.rs`.

## Public examples

- `admission_gate_8(4, 0)` gives `false`.
- `admission_gate_8(13, 1)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-190/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-190`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
