# task-2512 — ABI flags · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Advanced FFI lab. Define a local extern "C" callback with fixed-width unsigned arguments and result. Drive it through a typed C-ABI function pointer. Observe zero for zero, one otherwise. During review explain why substituting a Rust-ABI function pointer would change the interface.

Return the sum of all observations, widening each one before adding. Use an extern "C" fn(u8) -> u32 callback. During review, point to the operation that enforces its contract.

```rust
pub fn abi_flags_total(input: &[u8]) -> u64
```

## Public examples

- `abi_flags_total(&[])` gives `0`.
- `abi_flags_total(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2512/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2512`.

[Module reading](https://doc.rust-lang.org/reference/items/external-blocks.html)
