# task-2534 — ABI endian swap · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Advanced FFI lab. Define a local extern "C" callback with fixed-width unsigned arguments and result. Drive it through a typed C-ABI function pointer. Observe the byte widened to u16 then byte-swapped. During review explain why substituting a Rust-ABI function pointer would change the interface.

Keep the first observation and every later one strictly above all earlier observations. Use an extern "C" fn(u8) -> u32 callback. During review, point to the operation that enforces its contract.

```rust
pub fn abi_endian_swap_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `abi_endian_swap_peaks(&[])` gives `vec![]`.
- `abi_endian_swap_peaks(&[4])` gives `vec![1024u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2534/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2534`.

[Module reading](https://doc.rust-lang.org/reference/items/external-blocks.html)
