# task-2934 — Stability band · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

An upgrade incident log records old and new latency ticks as consecutive bytes for each probe. Ignore a final incomplete pair. Observe one if latencies differ by at most two ticks, else zero per complete pair. Provide the smallest probe that triggered the starter fault and a regression test that protects it.

Keep the first observation and every later one strictly above all earlier observations. Use paired measurements, widening or saturation appropriate to the metric. During review, point to the operation that enforces its contract.

```rust
pub fn stability_band_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `stability_band_peaks(&[])` gives `vec![]`.
- `stability_band_peaks(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2934/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2934`.

[Module reading](https://doc.rust-lang.org/cargo/reference/resolver.html)
