# task-184 — Even batch · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Require an even value at least 5.

```rust
pub fn even_batch_5(issued: u16, returned: u16) -> bool
```

Implement the public function in `src/lib.rs`.

## Public examples

- `even_batch_5(4, 0)` gives `false`.
- `even_batch_5(13, 1)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-184/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-184`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
