# task-222 — Skip reserved seats · two deliveries

Combine two deliveries, capping their sum at 255. Use the combined count as value. Count seats from 1 through value, skipping every Nth seat.

```rust
pub fn skip_reserved_seats_3(first: u8, second: u8) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `skip_reserved_seats_3(4, 0)` gives `3`.
- `skip_reserved_seats_3(13, 3)` gives `11`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-222/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-222`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
