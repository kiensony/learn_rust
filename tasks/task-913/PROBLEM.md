# task-913 — Visit readings with a stateful closure

Invoke visit on each value exactly once in input order. Sum its results as u64. A visitor may mutate captured state.

```rust
pub fn visit_total(values: &[u8], mut visit: impl FnMut(u8) -> u32) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `visit_total(&[2,3], |n| u32::from(n))` gives `5`.
- `visit_total(&[2,3], |n| u32::from(n)*2)` gives `10`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-913/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-913`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
