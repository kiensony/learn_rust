# task-187 — Three shelves · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Return 0 below 5, 1 from 5 through 7 inclusive, and 2 above 7.

```rust
pub fn three_shelves_5(issued: u16, returned: u16) -> u8
```

Implement the public function in `src/lib.rs`.

## Public examples

- `three_shelves_5(4, 0)` gives `0`.
- `three_shelves_5(13, 1)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-187/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-187`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
