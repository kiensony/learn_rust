# task-1205 — Line byte lengths · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Read a UTF-8 text stream line by line. Observe the byte length of each line after removing its LF or CRLF terminator. A final terminator ends the last line. Preserve interior empty lines.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use BufRead::lines with a Cursor over the input. During review, point to the operation that enforces its contract.

```rust
pub fn line_byte_lengths_changes(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `line_byte_lengths_changes("")` gives `vec![]`.
- `line_byte_lengths_changes("a")` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1205/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1205`.

[Module reading](https://doc.rust-lang.org/std/io/)
