# task-2284 — Profile repeated text scans

Count every LF byte, including consecutive LFs, without decoding UTF-8. Use the provided fixed workload to capture a CPU profile on Linux. Run cargo build --release --example measure, then perf record --call-graph dwarf -- target/release/examples/measure, followed by perf report --stdio. If perf events are unavailable, use the workload's elapsed time and record that limitation. Compare sample counts and output before and after the repair. Acceptance tests check deterministic results. Timing observations belong in the review.

```rust
pub fn find_newlines(input: &[u8]) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `find_newlines(&[])` gives `0`.
- `find_newlines(b"a\nb")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2284/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2284`.

[Module reading](https://doc.rust-lang.org/rustc/instrument-coverage.html)
