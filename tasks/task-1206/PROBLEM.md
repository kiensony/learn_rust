# task-1206 — Line byte lengths · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Read a UTF-8 text stream line by line. Observe the byte length of each line after removing its LF or CRLF terminator. A final terminator ends the last line. Preserve interior empty lines.

Count adjacent pairs for which the second observation is strictly smaller. Use BufRead::lines with a Cursor over the input. During review, point to the operation that enforces its contract.

```rust
pub fn line_byte_lengths_descending_steps(input: &str) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `line_byte_lengths_descending_steps("")` gives `0`.
- `line_byte_lengths_descending_steps("a")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1206/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1206`.

[Module reading](https://doc.rust-lang.org/std/io/)
