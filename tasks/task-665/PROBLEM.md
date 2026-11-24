# task-665 — Read a transport signal

Report zero for Go, the contained seconds for Wait, and u16::MAX for Closed. Match all three variants explicitly.

```rust
pub fn wait_seconds(signal: Signal) -> u16
```

Implement the public function in `src/lib.rs`.

## Public examples

- `wait_seconds(Signal::Go)` gives `0`.
- `wait_seconds(Signal::Wait(4))` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-665/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-665`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
