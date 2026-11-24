# task-106 — Bundle labels · limited supply

A request is limited by available stock. Use the smaller count as value. Each packet needs 10 item labels plus one address label. Count labels for value packets.

```rust
pub fn bundle_labels_10(requested: u16, available: u16) -> u32
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `bundle_labels_10(65535, 4)` gives `44`.
- `bundle_labels_10(13, 14)` gives `143`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-106/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-106`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
