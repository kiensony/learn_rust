# task-2634 — Embedded saturation · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Adapt a provided no_std device primitive to a host-side trace. For each byte double each u8 using saturation by calling track_device::convert. Keep allocation in the host adapter. Check the device with cargo check --manifest-path device/Cargo.toml --target thumbv7em-none-eabi after installing that target.

Keep the first observation and every later one strictly above all earlier observations. Use the local no_std device library and a separate host test harness. During review, point to the operation that enforces its contract.

```rust
pub fn embedded_saturation_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `embedded_saturation_peaks(&[])` gives `vec![]`.
- `embedded_saturation_peaks(&[4])` gives `vec![8u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2634/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2634`.

[Module reading](https://doc.rust-lang.org/reference/names/preludes.html)
