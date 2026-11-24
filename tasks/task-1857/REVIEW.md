# Review — Shared read handles · distinct

- Demonstrate the two public examples and the edge cases for `shared_read_handles_distinct`.
- Show the private helper, explain the duplicated or partial behavior it replaced, and check callers.
- Apply the module constraints in PROBLEM.md. Preserve the supplied tests and API.

## Submission

Make `// rust-track: handwritten task-1857` the first line of `src/lib.rs` and replace the starter’s zero marker with `pub const TRACK_PROOF: u32 = 0x52540000 | 1857;`.

Run `cargo track test task-1857` for the full suite.
