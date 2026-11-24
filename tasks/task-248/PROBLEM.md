# task-248 — Countdown checksum · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Start at value and repeatedly subtract 8 while positive. Sum the positive numbers visited.

```rust
pub fn countdown_checksum_8(start: u8, end: u8) -> u32
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `countdown_checksum_8(4, 0)` gives `4`.
- `countdown_checksum_8(13, 1)` gives `16`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-248/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-248`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
