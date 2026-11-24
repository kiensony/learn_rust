# task-238 — Countdown checksum · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Start at value and repeatedly subtract 5 while positive. Sum the positive numbers visited.

```rust
pub fn countdown_checksum_5(issued: u8, returned: u8) -> u32
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `countdown_checksum_5(4, 0)` gives `4`.
- `countdown_checksum_5(13, 1)` gives `21`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-238/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-238`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
