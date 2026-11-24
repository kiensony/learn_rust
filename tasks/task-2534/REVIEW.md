# Review — ABI endian swap · peaks

- Demonstrate the two public examples and the edge cases for `abi_endian_swap_peaks`.
- Show the private helper, explain the duplicated or partial behavior it replaced, and check callers.
- Apply the module constraints in PROBLEM.md. Preserve the supplied tests and API.

## Submission

Make `// rust-track: handwritten task-2534` the first line of `src/lib.rs` and replace the starter’s zero marker with `pub const TRACK_PROOF: u32 = 0x52540000 | 2534;`.

Run `cargo track test task-2534` for the full suite.
