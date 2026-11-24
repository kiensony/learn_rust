# task-133 — Round down · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Return the greatest multiple of 5 no larger than value.

```rust
pub fn round_down_5(issued: u16, returned: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `round_down_5(4, 0)` gives `0`.
- `round_down_5(13, 1)` gives `10`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-133/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-133`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
