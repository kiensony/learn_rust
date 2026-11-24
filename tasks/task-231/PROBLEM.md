# task-231 — Sum chimes · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Sum the positive multiples of 5 no greater than value.

```rust
pub fn sum_chimes_5(issued: u8, returned: u8) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `sum_chimes_5(4, 0)` gives `0`.
- `sum_chimes_5(13, 1)` gives `15`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-231/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-231`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
