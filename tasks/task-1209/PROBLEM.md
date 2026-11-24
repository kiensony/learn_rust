# task-1209 — Line byte lengths · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Read a UTF-8 text stream line by line. Observe the byte length of each line after removing its LF or CRLF terminator. A final terminator ends the last line. Preserve interior empty lines.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use BufRead::lines with a Cursor over the input. During review, point to the operation that enforces its contract.

```rust
pub fn line_byte_lengths_longest_run(input: &str) -> u64
```

## Public examples

- `line_byte_lengths_longest_run("")` gives `0`.
- `line_byte_lengths_longest_run("a")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1209/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1209`.

[Module reading](https://doc.rust-lang.org/std/io/)
