# task-2539 — ABI endian swap · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Advanced FFI lab. Define a local extern "C" callback with fixed-width unsigned arguments and result. Drive it through a typed C-ABI function pointer. Observe the byte widened to u16 then byte-swapped. During review explain why substituting a Rust-ABI function pointer would change the interface.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use an extern "C" fn(u8) -> u32 callback. During review, point to the operation that enforces its contract.

```rust
pub fn abi_endian_swap_longest_run(input: &[u8]) -> u64
```

## Public examples

- `abi_endian_swap_longest_run(&[])` gives `0`.
- `abi_endian_swap_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2539/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2539`.

[Module reading](https://doc.rust-lang.org/reference/items/external-blocks.html)
