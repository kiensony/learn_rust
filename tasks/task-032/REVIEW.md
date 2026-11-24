# Review for task-032

- Run the public examples and the full test suite. Check the learner's editable files against the behavior in PROBLEM.md.
- Inspect the compile-time lookup of CARGO_PKG_NAME. A future package rename must flow through to the display.
- Use safe Rust and the provided standard library. Keep the acceptance tests intact.

## Submission step

After the public examples pass, open `src/lib.rs`. Add the following first line and replace the placeholder constant with the second line below. These identify this task during course review.

```rust
// rust-track: handwritten task-032
pub const TRACK_PROOF: u32 = 0x52540000 | 32;
```
