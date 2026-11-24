# task-2548 — ABI signed decode · rolling load

Advanced FFI lab. Define a local extern "C" callback with fixed-width unsigned arguments and result. Drive it through a typed C-ABI function pointer. Observe the absolute magnitude of the byte interpreted as signed two's complement. During review explain why substituting a Rust-ABI function pointer would change the interface.

For each complete sliding window of width observations, return its sum as u64. Consecutive windows overlap by width minus one observations. Width zero and widths greater than the trace length yield an empty vector.

Use an extern "C" fn(u8) -> u32 callback. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn abi_signed_decode_rolling_load(input: &[u8], width: usize) -> Vec<u64>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `abi_signed_decode_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1)` gives `vec![3u64, 6u64, 9u64, 12u64, 21u64, 15u64, 0u64, 26u64, 8u64, 31u64]`.
- `abi_signed_decode_rolling_load(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2548/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2548`.

[Module reading](https://doc.rust-lang.org/reference/items/external-blocks.html)
