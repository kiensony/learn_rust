# task-2533 — ABI endian swap · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Advanced FFI lab. Define a local extern "C" callback with fixed-width unsigned arguments and result. Drive it through a typed C-ABI function pointer. Observe the byte widened to u16 then byte-swapped. During review explain why substituting a Rust-ABI function pointer would change the interface.

Return Some((minimum, maximum)) or None for an empty trace. Use an extern "C" fn(u8) -> u32 callback. During review, point to the operation that enforces its contract.

```rust
pub fn abi_endian_swap_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `abi_endian_swap_bounds(&[])` gives `None`.
- `abi_endian_swap_bounds(&[4])` gives `Some((1024u32, 1024u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2533/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2533`.

[Module reading](https://doc.rust-lang.org/reference/items/external-blocks.html)
