# task-2344 — Benchmark a rolling checksum

Compute a u32 checksum starting at zero. For each byte rotate the accumulator left by three bits then XOR the byte. Profile the fixed workload in examples/measure.rs with cargo run --example measure and cargo run --release --example measure. Keep workload and inputs identical. Record the median of five release runs separately from compilation time, and explain what black_box protects in this benchmark. Acceptance tests check deterministic results. Timing observations belong in the review.

```rust
pub fn checksum(input: &[u8]) -> u32
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `checksum(&[])` gives `0`.
- `checksum(&[1])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2344/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2344`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
