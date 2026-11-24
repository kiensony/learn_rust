# task-026 — Restore the program entry point

This crate builds a terminal program. Its entry point must have Rust's reserved entry-point name. Repair it so the program displays `Beacon on` on one line.

From the repository directory, open `tasks/task-026/` and edit `src/main.rs`. The example commands below run inside that task directory. Reproduce the symptom first. Use the first diagnostic or failing example to isolate the responsible statement. Make one small repair, then rerun the same case and the acceptance tests.

```sh
cargo test --manifest-path tasks/task-026/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-026/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run` produces standard output `"Beacon on\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-026/Cargo.toml
```
