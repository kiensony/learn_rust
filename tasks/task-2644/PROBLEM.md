# task-2644 — Embedded nibble swap · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Adapt a provided no_std device primitive to a host-side trace. For each byte swap the high and low nibbles by calling track_device::convert. Keep allocation in the host adapter. Check the device with cargo check --manifest-path device/Cargo.toml --target thumbv7em-none-eabi after installing that target.

Keep the first observation and every later one strictly above all earlier observations. Use the local no_std device library and a separate host test harness. During review, point to the operation that enforces its contract.

```rust
pub fn embedded_nibble_swap_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `embedded_nibble_swap_peaks(&[])` gives `vec![]`.
- `embedded_nibble_swap_peaks(&[4])` gives `vec![64u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2644/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2644`.

[Module reading](https://doc.rust-lang.org/reference/names/preludes.html)
