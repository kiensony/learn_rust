# task-232 — Skip reserved seats · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Count seats from 1 through value, skipping every Nth seat.

```rust
pub fn skip_reserved_seats_5(issued: u8, returned: u8) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `skip_reserved_seats_5(4, 0)` gives `4`.
- `skip_reserved_seats_5(13, 1)` gives `10`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-232/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-232`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
