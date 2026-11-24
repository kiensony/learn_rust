# Review for task-054

- Run the public examples and the full test suite. Check the learner's editable files against the behavior in PROBLEM.md.
- Inspect both executable source files. Both must include the same prompt file and preserve its final newline.
- Use safe Rust and the provided standard library. Keep the acceptance tests intact.
- Inspect both call sites and the extracted item. The shared item must supply the behavior used by the executable, including future changes.

## Submission step

After the public examples pass, open `src/lib.rs`. Add the following first line and replace the placeholder constant with the second line below. These identify this task during course review.

```rust
// rust-track: handwritten task-054
pub const TRACK_PROOF: u32 = 0x52540000 | 54;
```
