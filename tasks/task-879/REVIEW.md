# Review — Access a module through its public interface

- Demonstrate the two public examples and the edge cases for `code_size`.
- Identify the first incorrect step and show a regression that would have caught it.
- Apply the module constraints in PROBLEM.md. Preserve the supplied tests and API.

## Submission

Make `// rust-track: handwritten task-879` the first line of `src/lib.rs` and replace the starter’s zero marker with `pub const TRACK_PROOF: u32 = 0x52540000 | 879;`.

Run `cargo track test task-879` for the full suite.
