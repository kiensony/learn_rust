# task-1455 — HTTP status fixture · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Build the public decoder used by an external integration test. Accept ASCII whitespace-separated three-digit statuses from 100 through 599. Ignore malformed tokens. Observe the hundred-class digit of each accepted code. Add a separate tests/consumer.rs that calls the crate as a downstream consumer and covers a malformed token among valid tokens.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use an integration test outside src and the crate's public API. During review, point to the operation that enforces its contract.

```rust
pub fn http_status_fixture_changes(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `http_status_fixture_changes("")` gives `vec![]`.
- `http_status_fixture_changes("200")` gives `vec![2u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1455/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1455`.

[Module reading](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
