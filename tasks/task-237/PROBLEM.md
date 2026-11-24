# task-237 — Cycle ticks · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. A wheel starts at position zero on a ring of 7 positions. Advance 5 places value times and return its final position.

```rust
pub fn cycle_ticks_5(issued: u8, returned: u8) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `cycle_ticks_5(4, 0)` gives `6`.
- `cycle_ticks_5(13, 1)` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-237/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-237`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
