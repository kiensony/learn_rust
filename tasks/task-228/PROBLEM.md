# task-228 — Countdown checksum · two deliveries

Combine two deliveries, capping their sum at 255. Use the combined count as value. Start at value and repeatedly subtract 3 while positive. Sum the positive numbers visited.

```rust
pub fn countdown_checksum_3(first: u8, second: u8) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `countdown_checksum_3(4, 0)` gives `5`.
- `countdown_checksum_3(13, 3)` gives `51`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-228/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-228`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
